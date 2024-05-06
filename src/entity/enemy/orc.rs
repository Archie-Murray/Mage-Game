use std::time::Duration;

use bevy::{ecs::query::QuerySingleError, prelude::*};
use crate::pathfinding::{AIPath, Grid};
use crate::{pathfinding::AITarget, player::Player};
use crate::entity::{health::Health, damage::DamageType, stats::{Stats, StatType}};

use super::*;
use rand::Rng;

fn get_player_pos(player_transform: Result<&Transform, QuerySingleError>) -> Option<Vec2> {
    match player_transform {
        Ok(transform) => Some(transform.translation.truncate()),
        Err(_) => None,
    }
}

#[derive(Component)]
pub struct OrcIdle;
#[derive(Component)]
pub struct OrcWander;
#[derive(Component)]
pub struct OrcChase;
#[derive(Component)]
pub struct OrcAttack;
#[derive(Component)]
pub struct OrcDeath;

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            idle_enter.before(idle_update),
            idle_update,
            wander_enter.before(wander_update),
            wander_update,
            chase_enter.before(chase_update),
            chase_update,
            attack_enter.before(attack_update),
            attack_update,
        ));
    }
}

fn idle_enter(
    mut enemies: Query<(&mut Enemy, &mut DirectionalAnimator, &mut AITarget), Added<OrcIdle>>
) {
    let mut rng = rand::thread_rng();
    for (mut enemy, mut anim, mut ai) in enemies.iter_mut() {
        enemy.enemy_state = EnemyState::Idle;
        enemy.action_timer = Timer::from_seconds(rng.gen_range(3.0..5.0), TimerMode::Once);
        anim.update_animation(AnimationType::Idle);
        ai.do_path_find = false;
        info!("Started orc idle with time {:?}", enemy.action_timer.duration());
    }
}

fn idle_update(
    time: Res<Time>,
    mut commands: Commands,
    mut orcs: Query<(Entity, &mut Enemy), With<OrcIdle>>
) {
    for (entity, mut enemy) in orcs.iter_mut() {
        enemy.action_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if enemy.action_timer.finished() {
            match enemy.state_transitions.idle_exit {
                EnemyState::Wander => { commands.entity(entity).insert(OrcWander); },
                _ => { info!("Found unhandled transition from idle {:?}", enemy.state_transitions.idle_exit); continue; },
            }
            commands.entity(entity).remove::<OrcIdle>();
        }
    }
}

fn wander_enter(
    mut commands: Commands,
    mut orcs: Query<(Entity, &mut Enemy, &mut DirectionalAnimator, &mut AITarget, &Transform, Option<&AIPath>), Added<OrcWander>>,
    grid: Res<Grid>
) {
    let mut rng = rand::thread_rng();
    for (entity, mut enemy, mut anim, mut ai, transform, path) in orcs.iter_mut() {
        ai.do_path_find = true;
        let angle = rng.gen_range(-2.0 * std::f32::consts::PI..2.0 * std::f32::consts::PI).to_radians();
        let pos = Vec2::new(transform.translation.x + angle.cos() * 100.0, transform.translation.y.sin() * 100.0);
        if let Some(grid_pos) = grid.sample_position(&(transform.translation.truncate() + pos).as_ivec2(), (pos - transform.translation.truncate()).normalize()) {
            if path.is_some() {
                commands.entity(entity).remove::<AIPath>();
            }
            ai.destination = Vec2::new(grid_pos.0 as f32, grid_pos.1 as f32);
            info!("Intialised orc with wander target: ({}, {})", grid_pos.0, grid_pos.1);
        }
        enemy.enemy_state = EnemyState::Wander;
        anim.update_animation(AnimationType::Walk);
    }
}

fn wander_update(
    grid: Res<Grid>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut orcs: Query<(Entity, &Enemy, &Transform, &AITarget, Option<&AIPath>), With<OrcWander>>
) {
    let Some(player_pos) = get_player_pos(player_query.get_single()) else {
        for (en, _, _, _, _) in orcs.iter() {
            commands.entity(en).remove::<OrcWander>().insert(OrcIdle);
        }
        return;
    };
    for (entity, enemy, transform, ai, path) in orcs.iter_mut() {
        if player_pos.distance_squared(transform.translation.truncate()) <= ai.follow_range.powi(2) {
            info!("Player in range!");
            if path.is_some() {
                commands.entity(entity).remove::<AIPath>();
            }
            match enemy.state_transitions.wander_chase {
                EnemyState::Chase => { commands.entity(entity).insert(OrcChase); },
                EnemyState::Attack => { commands.entity(entity).insert(OrcAttack); },
                _ => { info!("Found unhandled transition from wander {:?}", enemy.state_transitions.wander_chase); continue; },
            }
            commands.entity(entity).remove::<OrcWander>();
        }
        if transform.translation.truncate().distance_squared(grid.grid_to_world_coords(&ai.destination.as_ivec2())) <= crate::pathfinding::GRID_TOLERANCE.powi(2) {
            if path.is_some() {
                commands.entity(entity).remove::<AIPath>();
            }
            match enemy.state_transitions.wander_idle {
                EnemyState::Idle => { commands.entity(entity).insert(OrcIdle); },
                _ => { info!("Found unhandled transition from wander {:?}", enemy.state_transitions.wander_idle); commands.entity(entity).insert(OrcIdle); },
            }
            commands.entity(entity).remove::<OrcWander>(); 
            continue;
        }
    }
}

fn chase_enter(
    mut anims: Query<(&mut DirectionalAnimator, &mut AITarget), Added<OrcChase>>
) {
    for (mut anim, mut ai) in anims.iter_mut() {
        ai.do_path_find = true;
        anim.update_animation(AnimationType::Walk);
    }
}

fn chase_update(
    grid: Res<Grid>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut orcs: Query<(Entity, &Enemy, &Transform, &mut AITarget, Option<&AIPath>), With<OrcChase>>
) {
    let Some(player_pos) = get_player_pos(player_query.get_single()) else {
        for (entity, _, _, _, _) in orcs.iter() {
            commands.entity(entity).remove::<OrcChase>().insert(OrcIdle);
        }
        return;
    };
    for (entity, enemy, transform, mut ai, path) in orcs.iter_mut() {
        let distance_to_player = transform.translation.truncate().distance(player_pos);
        if distance_to_player >= ai.follow_range {
            if path.is_some() {
                match enemy.state_transitions.chase_exit {
                    EnemyState::Wander => { commands.entity(entity).insert(OrcWander); },
                    _ => { info!("Found unhandled transition from chase exit {:?}", enemy.state_transitions.chase_exit); commands.entity(entity).insert(OrcIdle); },
                }
                commands.entity(entity).remove::<OrcChase>();
            }
            continue;
        }
        if grid.grid_to_world_coords(&ai.destination.as_ivec2()).distance_squared(player_pos) >= crate::pathfinding::GRID_TOLERANCE.powi(2) {
            if let Some((pos_x, pos_y)) = grid.sample_position(&player_pos.as_ivec2(), (player_pos - transform.translation.truncate()).normalize()) {
                commands.entity(entity).remove::<AIPath>();
                ai.destination = Vec2::new(pos_x as f32, pos_y as f32);
            } 
        }
        if distance_to_player <= ai.attack_range {
            match enemy.state_transitions.chase_player {
                EnemyState::Attack => { commands.entity(entity).insert(OrcAttack); },
                _ => { info!("Found unhandled transition from chase player {:?}", enemy.state_transitions.chase_player); commands.entity(entity).insert(OrcIdle); },
            }
            commands.entity(entity).remove::<OrcChase>();
        }
    }
}

fn attack_enter(
    mut player_query: Query<&mut Health, With<Player>>,
    mut orcs: Query<(&mut Enemy, &mut DirectionalAnimator, &Stats), Added<OrcAttack>>
) {
    let Some(mut player_health) = (match player_query.get_single_mut() {
        Ok(health) => Some(health),
        Err(_) => None,
    }) else { return; };
    for (mut enemy, mut animator, stats) in orcs.iter_mut() {
        animator.update_animation(AnimationType::Attack);
        enemy.enemy_state = EnemyState::Attack;
        enemy.action_timer = Timer::from_seconds(1.0, TimerMode::Once);
        let Some(damage) = stats.get_stat(StatType::Attack) else { continue; };
        player_health.push_damage(*damage, DamageType::PHYSICAL);
    }
}

fn attack_update(
    time: Res<Time>,
    mut commands: Commands,
    mut orcs: Query<(Entity, &mut Enemy), With<OrcAttack>>
) {
    for (entity, mut enemy) in orcs.iter_mut() {
        enemy.action_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if enemy.action_timer.finished() {
            match enemy.state_transitions.attack_finished {
                EnemyState::Chase => { commands.entity(entity).insert(OrcChase); },
                _ => { info!("Found unhandled transition from attack exit {:?}", enemy.state_transitions.attack_finished); continue; }
            }
            commands.entity(entity).remove::<OrcAttack>();
        }
    }
}

// pub fn update_orc(
//     delta: f32, 
//     enemy: &mut Enemy, 
//     enemy_transform: &Transform, 
//     enemy_stats: &mut Stats, 
//     enemy_ai: &AITarget, 
//     enemy_animator: &mut DirectionalAnimator, 
//     player_transform: &Transform, 
//     player_health: &mut Health
// ) {
//     match enemy.enemy_state {
//         EnemyState::Idle => idle(delta, enemy, enemy_transform, enemy_stats, enemy_ai, enemy_animator, player_transform, player_health),
//         EnemyState::Wander => wander(delta, enemy, enemy_transform, enemy_stats, enemy_ai, enemy_animator, player_transform, player_health),
//         EnemyState::Chase => chase(delta, enemy, enemy_transform, enemy_stats, enemy_ai, enemy_animator, player_transform, player_health),
//         EnemyState::Attack => attack(delta, enemy, enemy_transform, enemy_stats, enemy_ai, enemy_animator, player_transform, player_health),
//         EnemyState::Death => death(delta, enemy, enemy_transform, enemy_stats, enemy_ai, enemy_animator, player_transform, player_health),
//     }
// }
//
// pub fn distance_to_player(player: Transform, enemy: Transform) -> f32 {
//     enemy.translation.distance(player.translation)
// }
//
// pub fn idle(
//     delta: f32, 
//     mut enemy: &Enemy, 
//     enemy_transform: &Transform, 
//     mut enemy_stats: &Stats, 
//     enemy_ai: &AITarget, 
//     mut enemy_animator: &DirectionalAnimator, 
//     player_transform: &Transform, 
//     mut player_health: &Health
// ) {
//     enemy_animator.update_animation(AnimationType::Idle);
// }
//
// pub fn wander(
//     delta: f32, 
//     mut enemy: &Enemy, 
//     enemy_transform: &Transform, 
//     mut enemy_stats: &Stats, 
//     enemy_ai: &AITarget, 
//     mut enemy_animator: &DirectionalAnimator, 
//     player_transform: &Transform, 
//     mut player_health: &Health
// ) {
//     enemy_animator.update_animation(AnimationType::Walk);
//     if distance_to_player(*player_transform, *enemy_transform) <= enemy_ai.follow_range {
//         enemy.enemy_state = EnemyState::Chase;
//     }
// }
//
// pub fn chase(
//     delta: f32, 
//     mut enemy: &Enemy, 
//     enemy_transform: &Transform, 
//     mut enemy_stats: &Stats, 
//     enemy_ai: &AITarget, 
//     mut enemy_animator: &mut DirectionalAnimator, 
//     player_transform: &Transform, 
//     mut player_health: &Health
// ) {
//     enemy_animator.update_animation(AnimationType::Walk);
//     let distance_to_player = distance_to_player(*player_transform, *enemy_transform);
//     if distance_to_player <= enemy_ai.attack_range {
//         enemy.enemy_state = EnemyState::Attack;
//     }
//     if distance_to_player >= enemy_ai.follow_range {
//         enemy.enemy_state = EnemyState::Wander;
//     }
// }
//
// pub fn attack(
//     delta: f32, 
//     mut enemy: &Enemy, 
//     enemy_transform: &Transform, 
//     mut enemy_stats: &Stats, 
//     enemy_ai: &AITarget, 
//     mut enemy_animator: &DirectionalAnimator, 
//     player_transform: &Transform, 
//     mut player_health: &Health
// ) {
//     if !enemy.anim_timer.finished() {
//         enemy_animator.update_animation(AnimationType::Attack);
//         let damage = *enemy_stats.get_stat(StatType::Attack).unwrap_or(&0.0);
//         player_health.damage(damage, crate::damage::damagetype::DamageType::PHYSICAL);
//         enemy.anim_timer = Timer::from_seconds(1.0, TimerMode::Once);
//     } else {
//         enemy.anim_timer.tick(Duration::from_secs_f32(delta));
//         if enemy.anim_timer.finished() {
//             enemy.enemy_state = if distance_to_player(*player_transform, *enemy_transform) >= enemy_ai.attack_range {
//                 EnemyState::Chase
//             } else { 
//                 EnemyState::Attack 
//             };
//         }
//     }
// }
//
// pub fn death(
//     delta: f32, 
//     mut enemy: &Enemy, 
//     enemy_transform: &Transform, 
//     mut enemy_stats: &Stats, 
//     enemy_ai: &AITarget, 
//     mut enemy_animator: &DirectionalAnimator, 
//     player_transform: &Transform, 
//     mut player_health: &Health
// ) {
//
// }

