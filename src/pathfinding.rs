use std::{collections::VecDeque, fs::File, io::Write};

use bevy::{
    prelude::*, render::primitives::Aabb, tasks::{AsyncComputeTaskPool, Task}
};
use bevy_rapier2d::prelude::*;
use futures_lite::future;
use pathfinding::prelude::astar;
use std::file;

use crate::{
    entity::{stat_type::StatType, stats::Stats},
    map::Wall,
};

const GRID_SIZE: i32 = 128;
const HALF_GRID_SIZE: i32 = GRID_SIZE / 2;
const TILE_SIZE: i32 = 16;

#[derive(Component, Reflect)]
pub struct AITarget {
    pub follow_range: f32,
    pub attack_range: f32,
    pub destination: Vec2,
    pub do_path_find: bool
}

impl AITarget {
    pub fn new(follow_range: f32, attack_range: f32) -> Self {
        AITarget {
            follow_range,
            attack_range,
            destination: Vec2::ZERO,
            do_path_find: false
        }
    }
}

#[derive(Component, Reflect)]
pub struct AIPath {
    pub index: usize,
    pub points: VecDeque<IVec2>,
}

pub struct Path {
    pub steps: Vec<IVec2>,
}

#[derive(Debug)]
pub struct PathfindingError;

#[derive(Component)]
pub struct PathfindingTask {
    pub task: Task<Result<Path, PathfindingError>>,
}

#[derive(Resource, Clone, Reflect)]
pub struct Grid {
    pub points: [[bool; GRID_SIZE as usize]; GRID_SIZE as usize]
}

pub fn populate_grid(mut grid: ResMut<Grid>, walls: Query<(&Transform, &Aabb), With<Wall>>) {
    let mut count = 0;
    for (transform, aabb) in walls.iter() {
        let top_left = (transform.translation.truncate() - aabb.half_extents.truncate()).as_ivec2();
        let bottom_right = (transform.translation.truncate() + aabb.half_extents.truncate()).as_ivec2();
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                *grid.index_mut(IVec2::new(x, y)) = true;
                count += 1;
            }
        }
    }
    info!("Initialised grid with {} wall tiles", count);
    let Ok(mut world) = File::create("world.txt") else {
        return;
    };
    for line in grid.points {
        let data = line.map(|x| if x { 1 } else { 0 }).iter().copied().collect::<Vec<i32>>();
        let _ = world.write_all(&format!("{:?}\n", data).into_bytes());
    }
}

pub fn neuman_neighbours(grid: &Grid, location: &IVec2) -> Vec<IVec2> {
    let (x, y) = (location.x as u32, location.y as u32);
    let mut sucessors = Vec::new();
    if let Some(left) = x.checked_sub(1) {
        let location = IVec2::new(left as i32, y as i32);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = IVec2::new(x as i32, down as i32);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if x + 1 < GRID_SIZE as u32 {
        let right = x + 1;
        let location = IVec2::new(right as i32, y as i32);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if y + 1 < GRID_SIZE as u32 {
        let up = y + 1;
        let location = IVec2::new(x as i32, up as i32);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    sucessors
}

impl Grid {
    pub fn path_to(&self, start: &IVec2, end: &IVec2) -> Result<Path, PathfindingError> {
        let result = astar(
            start,
            |p| {
                neuman_neighbours(self, p)
                    .iter()
                    .map(|neighbour| (neighbour.clone(), 1))
                    .collect::<Vec<_>>()
            },
            |p| (p.as_vec2().distance(end.as_vec2())).round() as i32 / 3,
            |p| p.as_vec2().distance_squared(end.as_vec2()) <= 25.0,
        );
        if let Some((steps, _length)) = result {
            Ok(Path { steps })
        } else {
            Err(PathfindingError)
        }
    }

    pub fn occupied(&self, point: &IVec2) -> bool {
        let (x, y) = Grid::sanitised_point(point);
        self.points[y][x]
    }

    pub fn index_mut(&mut self, point: IVec2) -> &mut bool {
        let (x, y) = Grid::sanitised_point(&point);
        &mut self.points[y][x]
    }

    pub fn sanitised_point(point: &IVec2) -> (usize, usize) {
        let world_offset_i32 = crate::WORLD_OFFSET.as_ivec2();
        let no_offset = *point - world_offset_i32;
        (
            remap_i32(no_offset.x, -1 * world_offset_i32.x.abs(), world_offset_i32.x.abs(), 0, GRID_SIZE - 1) as usize,
            remap_i32(no_offset.y, -1 * world_offset_i32.y.abs(), world_offset_i32.y.abs(), 0, GRID_SIZE - 1) as usize
        )
    }
}

pub fn remap_i32(current: i32, old_min: i32, old_max: i32, new_min: i32, new_max: i32) -> i32 {
    lerp_i32(new_min, new_max, inv_lerp_i32(old_min, old_max, current))
}

pub fn lerp_i32(min: i32, max: i32, scalar: f32) -> i32 {
    ((1.0 - scalar.clamp(0.0, 1.0)) * (min + max) as f32 * scalar).round() as i32
}

pub fn inv_lerp_i32(min: i32, max: i32, current: i32) -> f32 {
    (current - min) as f32 / (max - min) as f32
}


impl Default for Grid {
    fn default() -> Self {
        Grid {
            points: [[false; GRID_SIZE as usize]; GRID_SIZE as usize],
        }
    }
}

pub fn spawn_optimized_pathfinding_task(
    commands: &mut Commands,
    target: Entity,
    grid: &Grid,
    start: IVec2,
    end: IVec2,
) {
    // Fail early if end is not valid
    if grid.occupied(&end) {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    // Must clone because the grid can change between frames
    // Must box to prevent stack overflows on very large grids
    let grid_clone = Box::new(grid.clone());

    let task = thread_pool.spawn(async move { grid_clone.path_to(&start, &end) });
    commands.entity(target).insert(PathfindingTask { task });
}

pub fn apply_pathfinding_to_ai(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.task)) {
            commands.entity(task_entity).remove::<PathfindingTask>();
            if let Ok(path) = result {
                let mut ai_path = AIPath { points: VecDeque::new(), index: 0usize };
                for location in path.steps.iter() {
                    ai_path.points.push_back(IVec2::new(location.x, location.y));
                }
                info!("Found path for entity: {} of {:?}", task_entity.index(), path.steps);
                commands.entity(task_entity).insert(ai_path);
            }
        }
    }
}

// NOTE: Convert to literally anything else!
pub fn traverse_path(mut ai_pathfinders: Query<(&mut Velocity, &AITarget, &Transform, &Stats, &mut AIPath)>) {
    for (mut pathfinder, target, transform, stats, mut ai) in ai_pathfinders.iter_mut() {
        if !target.do_path_find { pathfinder.linvel = Vec2::ZERO; continue; }
        let speed = *(stats.get_stat(StatType::Speed).unwrap_or(&100.0));
        if ai.points[ai.index].as_vec2().distance_squared(transform.translation.truncate()) <= 1.0 && ai.index < ai.points.len() - 1 {
            ai.index += 1;
        } else {
            return;
        }
        pathfinder.linvel = (ai.points[ai.index].as_vec2() - transform.translation.truncate()).normalize() * speed;
    }
}

pub fn update_ai_destinations(
    mut commands: Commands,
    mut pathfinders: Query<(Entity, &AITarget, &AIPath)>,
) {
    for (entity, ai_target, path) in pathfinders.iter_mut() {
        if !ai_target.do_path_find { continue; }
        if path
            .points
            .iter()
            .last()
            .unwrap()
            .distance_squared(ai_target.destination.as_ivec2())
            >= 100
        {
            info!("Need to recalculate path as last point is {} px away!", path.points.iter().last().unwrap().distance_squared(ai_target.destination.as_ivec2()));
            commands.entity(entity).remove::<AIPath>();
        }
    }
}

pub fn calculate_paths(
    mut commands: Commands,
    grid: Res<'_, Grid>,
    pathfinders: Query<
        (Entity, &Transform, &AITarget),
        (Without<AIPath>, Without<PathfindingTask>),
    >,
) {
    for (entity, transform, target) in pathfinders.iter() {
        spawn_optimized_pathfinding_task(
            &mut commands,
            entity,
            &grid,
            transform.translation.truncate().as_ivec2(),
            target.destination.as_ivec2(),
        );
    }
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                traverse_path,
                apply_pathfinding_to_ai,
                //update_ai_destinations,
                calculate_paths,
            ),
        );
        app.register_type::<AITarget>();
        app.register_type::<AIPath>();
    }
}
#[cfg(test)]
mod tests {

    use bevy::prelude::*;

    use crate::pathfinding::Grid;

    #[test]
    fn basic_pathfinding() {
        let goal = IVec2::new(4, 6);
        let start = IVec2::new(1, 1);
        let mut grid = Grid::default();
        grid.points[2][0] = false;
        grid.points[2][1] = false;
        grid.points[2][2] = false;

        let result = grid.path_to(&start, &goal);
        assert!(result.is_ok());
    }
}
