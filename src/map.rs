use std::io::Write;

use bevy::prelude::*;
use rand::Rng;
use crate::{pathfinding::Grid, WORLD_SIZE};
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Reflect)]
pub enum WallType {
    Circle(f32),
    Rect(f32)
}

#[derive(Component)]
pub struct Wall {
    pub wall_type: WallType
}

#[derive(Component)]
pub struct Void;

#[derive(Component)]
pub struct Ground;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_map_collision.after(spawn_map)));
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    commands.spawn(SpriteBundle {
        texture: asset_server.load("environment/ground.png"),
        transform: Transform::from_xyz(0.0, 0.0, -9.0),
        ..default()
    });

    let angles: [f32; 4] = [
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
    ];

    for angle in angles {
        let distance = rng.gen_range(256.0..384.0);
        let radius: f32 = match angle.round() as i32 / 90 {
            0 => 32.0,
            1 => 24.0,
            2 => 16.0,
            3 => 24.0,
            _ => 32.0
        };
        let rock_sprite = SpriteBundle {
            texture: asset_server.load(format!("environment/rock{}.png", angle.round() as i32 / 90)),
            transform: Transform::from_xyz(distance * angle.to_radians().sin(), distance * angle.to_radians().cos(), -9.0),
            ..default()
        };
        commands
            .spawn(rock_sprite)
            .insert(Collider::ball(radius))
            .insert(Wall { wall_type: WallType::Circle(radius) });
    }
}

fn spawn_map_collision(
    mut grid: ResMut<Grid>,
    walls: Query<(&Transform, &Wall)>
) {
    let mut count = 0;
    let dim = grid.dimensions();
    for (transform, wall) in walls.iter() {
        match wall.wall_type {
            WallType::Circle(radius) => {
                let indices = grid.index_from_position(&transform.translation.truncate().as_ivec2());
                let u_radius = radius.remap(0.0, WORLD_SIZE.x as f32, 0.0, dim.0 as f32).round() as usize;
                info!("Filling circle with u_radius {} from radius {}", u_radius, radius);
                fill_circle(&mut grid, indices, u_radius);
                count += 1;
            },
            WallType::Rect(_) => {
                info!("Not implemented!");
            }
        };
    }
    let dim = grid.dimensions();
    if dim.0 == dim.1 { // Wrap in walls
        for i in 0..dim.0 {
            grid.set_point(i, 0, true);
            grid.set_point(0, i, true);
            grid.set_point(dim.0, i, true);
            grid.set_point(i, dim.1, true);
        }
    }
    if let Ok(mut file) = std::fs::File::create("world.world") {
        for row in grid.points {
            let buf = row.iter().map(|is_wall| if *is_wall { String::from("██") } else { String::from("  ") }).collect::<Vec<String>>();
            let _ = file.write_all(format!("{}\n", buf.join("")).as_bytes());
        }
    }
    info!("Constructed map with wall count: {}", count);
}

fn fill_circle(grid: &mut ResMut<'_, Grid>, indices: (usize, usize), radius: usize) {
    for y in (indices.1.checked_sub(radius).unwrap_or(0))..(indices.1 + radius) {
        for x in (indices.0.checked_sub(radius).unwrap_or(0))..(indices.0 + radius) {
            if IVec2::new(indices.0 as i32, indices.1 as i32).distance_squared(IVec2::new(x as i32, y as i32)) <= (radius * radius) as i32 {
                grid.set_point(x, y, true);
            }
        }
    }
}
