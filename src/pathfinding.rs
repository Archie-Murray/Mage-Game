use std::collections::VecDeque;

use bevy::{
    prelude::*, tasks::{AsyncComputeTaskPool, Task}
};
use bevy_rapier2d::prelude::*;
use futures_lite::future;
use pathfinding::prelude::astar;

use crate::{
    entity::stats::{Stats, StatType},
    WORLD_SIZE,
};

pub const GRID_SIZE: i32 = 512;
pub static GRID_TOLERANCE: f32 = (WORLD_SIZE.x / GRID_SIZE) as f32;

#[derive(Component, Reflect)]
pub struct AITarget {
    pub follow_range: f32,
    pub attack_range: f32,
    pub destination: Vec2,
    pub do_path_find: bool
}

impl AITarget {
    pub fn new(follow_range: f32, attack_range: f32, start_pathfinding: bool) -> Self {
        AITarget {
            follow_range,
            attack_range,
            destination: Vec2::ZERO,
            do_path_find: start_pathfinding
        }
    }
}

#[derive(Component, Reflect)]
pub struct AIPath {
    pub index: usize,
    pub points: VecDeque<IVec2>,
}

impl AIPath {
    pub fn get_target(&self) -> IVec2 {
        self.points[self.index]
    }

    pub fn get_target_world(&self, grid: &Grid) -> Vec2 {
        grid.grid_to_world_coords(&self.get_target())
    }
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
            |p| p.as_vec2().distance_squared(end.as_vec2()) <= 2.0,
        );
        if let Some((mut steps, _length)) = result {
            steps.push(*end);
            Ok(Path { steps })
        } else {
            Err(PathfindingError)
        }
    }

    pub fn occupied(&self, point: &IVec2) -> bool {
        let (x, y) = self.index_from_position(&point);
        self.points[y][x]
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.points.len(), self.points[0].len())
    }

    pub fn index_from_position(&self, pos: &IVec2) -> (usize, usize) {
        let point = *pos;
        let (dim_y, dim_x) = self.dimensions();
        let (dim_y, dim_x) = (dim_y as i32, dim_x as i32);
        let half_world = WORLD_SIZE / 2;
        (
            remap_i32(point.x, -1 * half_world.x.abs(), half_world.x.abs(), 0, dim_x - 1).clamp(0, dim_x - 1) as usize,
            remap_i32(point.y, -1 * half_world.y.abs(), half_world.y.abs(), 0, dim_y - 1).clamp(0, dim_y - 1) as usize
        )

    }

    pub fn set_point(&mut self, x: usize, y: usize, value: bool) {
        let (dim_y, dim_x) = (self.points.len(), self.points[0].len());
        self.points[y.clamp(0, dim_y - 1)][x.clamp(0, dim_x - 1)] = value;
    }

    pub fn sample_position(&self, pos: &IVec2, dir: Vec2) -> Option<(usize, usize)> {
        let pos = self.index_from_position(pos);
        let dim = self.dimensions();
        if self.occupied(&IVec2::new(pos.0 as i32, pos.1 as i32)) {
            for y in pos.1..(if dir.y.signum() == 1.0 { dim.1 } else { 0usize }) { //This is dumb but I can't think of a better way
                for x in pos.0..(if dir.x.signum() == 1.0 { dim.0 } else { 0usize }) { //Same but magnified
                    if self.occupied(&IVec2::new(x as i32, y as i32)) {
                        return Some((x, y));
                    }
                }
            };
            None
        } else {
            Some(pos)
        }
    }

    pub fn grid_to_world_coords(&self, pos: &IVec2) -> Vec2 {
        let pos = pos.as_vec2();
        let half_world = WORLD_SIZE.as_vec2() / 2.0;
        let (dim_y, dim_x) = self.dimensions();
        let (dim_y, dim_x) = (dim_y as f32, dim_x as f32);
        Vec2::new(
            pos.x.remap(0.0, dim_x, -half_world.x, half_world.x),
            pos.y.remap(0.0, dim_y, -half_world.y, half_world.y),
        )
    }
}

pub fn remap_i32(current: i32, old_min: i32, old_max: i32, new_min: i32, new_max: i32) -> i32 {
    lerp_i32(new_min, new_max, inv_lerp_i32(old_min, old_max, current))
}

pub fn lerp_i32(min: i32, max: i32, scalar: f32) -> i32 {
    (min as f32 + (max - min).abs() as f32 * scalar).round() as i32
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
                commands.entity(task_entity).insert(ai_path);
            }
        }
    }
}

pub fn traverse_path(mut ai_pathfinders: Query<(&mut Velocity, &AITarget, &Transform, &Stats, &mut AIPath)>, grid: Res<Grid>) {
    for (mut pathfinder, target, transform, stats, mut ai) in ai_pathfinders.iter_mut() {
        if !target.do_path_find { pathfinder.linvel = Vec2::ZERO; continue; }
        let speed = *(stats.get_stat(StatType::Speed).unwrap_or(&100.0));
        if ai.get_target_world(&grid).distance_squared(transform.translation.truncate()) <= GRID_TOLERANCE && ai.index < ai.points.len() - 1 {
            ai.index += 1;
        }
        pathfinder.linvel = (ai.get_target_world(&grid) - transform.translation.truncate()).normalize_or_zero() * speed;
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
        let Some((pos_x, pos_y)) = grid.sample_position(&transform.translation.truncate().as_ivec2(), Vec2::ZERO) else { return; };
        if !target.do_path_find { continue; }
        spawn_optimized_pathfinding_task(
            &mut commands,
            entity,
            &grid,
            IVec2::new(pos_x as i32, pos_y as i32),
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

    use super::Grid;

    #[test]
    pub fn test_remap() {
        let coords = IVec2::ZERO;
        let grid = Grid::default();
        let dimensions = grid.dimensions();
        let new_coords = grid.index_from_position(&coords);
        assert!(new_coords == (dimensions.0 / 2, dimensions.1 / 2), "({}, {}) was not midpoint ({}, {})", new_coords.0, new_coords.1, dimensions.0, dimensions.1);
    }

    #[test]
    pub fn rest_world_coords() {
        let grid = Grid::default();
        let (x, y) = grid.dimensions();
        println!("Grid dimensions {}, {} and testing centre point: (63, 63)", x, y);
        let center = (x / 2, y / 2);
        let test_point = grid.grid_to_world_coords(&IVec2::new(x as i32 / 2, y as i32 / 2));
        assert!(test_point.distance_squared(Vec2::ZERO) <= 10.0, "100% Rust bug not mine ;) {:?} (center: {},{})", test_point, center.0, center.1);
    }
}
