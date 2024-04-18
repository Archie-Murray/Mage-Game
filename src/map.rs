use bevy::{prelude::*, render::render_resource::AsBindGroup, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};
use rand::Rng;
use crate::damage::{damagetype::DamageType, health::Health};
use bevy_rapier2d::prelude::*;

static MAP_WIDTH: IVec2 = IVec2 { x: 512, y: 512 };

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Void;

#[derive(Component)]
pub struct Ground;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<Perlin2dMaterial>::default())
            .add_systems(Startup, spawn_map)
            .add_systems(FixedUpdate, (do_void_damage, update_void_shader))
        ;
    }
}

// Set up materials
#[derive(AsBindGroup, Debug, Clone, Reflect, Asset)]
pub struct Perlin2dMaterial {
    #[uniform(0)]
    time: f32
}

impl Material2d for Perlin2dMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/perlin2d_material.wgsl".into()
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut perlin_materials: ResMut<Assets<Perlin2dMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mut rng = rand::thread_rng();

    let void = MaterialMesh2dBundle {
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(Rectangle::default()))),
        material: perlin_materials.add(Perlin2dMaterial { time: 0.0 }),
        transform: Transform::from_scale(Vec3::splat(2048.0)).with_translation(Vec2::new(MAP_WIDTH.x as f32 / 2.0, 0.0).extend(-11.0)),
        ..default()
    };

    commands
        .spawn(void)
        .insert(Void);

    let ground = SpriteBundle {
        texture: asset_server.load("environment/ground.png"),
        transform: Transform::from_translation((0.5 * MAP_WIDTH.as_vec2()).extend(-10.0)),
        ..default()
    };
    commands
        .spawn(ground)
        .insert(Collider::ball(MAP_WIDTH.x as f32))
        .insert(Ground)
        .insert(Sensor);

    let angles: [f32; 4] = [
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
        rng.gen_range(0.0..360.0),
    ];

    for angle in angles {
        let distance = rng.gen_range(MAP_WIDTH.x as f32 * 0.5..MAP_WIDTH.x as f32 * 0.75);
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
            .insert(Wall);
    }
}

fn do_void_damage(
    rapier: Res<RapierContext>,
    mut health_entities: Query<(Entity, &mut Health), With<Collider>>,
    ground_q: Query<Entity, (With<Ground>, Without<Health>)>
) {
    let Ok(ground) = ground_q.get_single() else { error!("Multiple ground entities?"); return; };
    for (health_entity, mut health) in health_entities.iter_mut() {
        if !rapier.intersection_pair(health_entity, ground).unwrap_or(false) {
            health.damage(0.1, DamageType::BYPASS);
        }
    }
}

fn update_void_shader(
    mut perlin_materials: ResMut<Assets<Perlin2dMaterial>>
) {
    for perlin in perlin_materials.iter_mut() {
        perlin.1.time += 0.02;
    }
}
