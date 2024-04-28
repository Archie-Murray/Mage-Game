use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hanabi::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ParticleType {
    Hit
}

pub struct ParticlePlugin;

// TODO: This does not work?
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Particles { effects: HashMap::new() });
        app.add_systems(Startup, init_particles);
    }
}

#[derive(Resource)]
pub struct Particles {
    pub effects: HashMap<ParticleType, Handle<EffectAsset>>
}

fn init_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<Particles> 
) {
    particle_effects.effects.insert(
        ParticleType::Hit, hit_effect(&mut effects)
    );
}

pub fn hit_effect(particle_effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
    let colour_grad = Gradient::linear(Vec4::new(1.0, 0.0, 0.0, 1.0), Vec4::ZERO);
    let size_grad = Gradient::linear(Vec2::ONE * 10.0, Vec2::ZERO);
    let writer = ExprWriter::new();
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::LIFETIME, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(15.0).uniform(writer.lit(30.0)).expr(),
        dimension: ShapeDimension::Surface
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(5.0).expr()
    };

    let module = writer.finish();

    particle_effects.add(
        EffectAsset::new(
            200, 
            Spawner::rate(20.0.into()), 
            module
        ).with_name("Hit Effect")
        .init(init_lifetime)
        .init(init_age)
        .init(init_vel)
        .init(init_pos)
        .render(ColorOverLifetimeModifier {
            gradient: colour_grad
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            screen_space_size: false
        })
    )
}
