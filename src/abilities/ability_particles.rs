use bevy_hanabi::prelude::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ParticleType { FireBall, IceStorm, HealOrb, FireBallDetonate, IceStormFinish, HealOrbDetonate }

#[derive(Resource)]
pub struct AbilityParticle {
    pub particle_effects: HashMap<ParticleType, Handle<EffectAsset>>
}

impl Default for AbilityParticle {
    fn default() -> Self {
        return AbilityParticle {
            particle_effects: HashMap::new()
        };
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilityParticle>();
        app.add_systems(Startup, init_particles);
    }
}

pub fn init_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particles: ResMut<AbilityParticle>
) {
    let ice_effects = create_ice_storm_particles(&mut effects);
    let fire_effects = create_fire_ball_particles(&mut effects);
    let heal_effects = create_heal_orb_particles(&mut effects);
    particles.particle_effects.insert(ParticleType::IceStorm, ice_effects.0);
    particles.particle_effects.insert(ParticleType::IceStormFinish, ice_effects.1);
    particles.particle_effects.insert(ParticleType::FireBall, fire_effects.0);
    particles.particle_effects.insert(ParticleType::FireBallDetonate, fire_effects.1);
    particles.particle_effects.insert(ParticleType::HealOrb, heal_effects.0);
    particles.particle_effects.insert(ParticleType::HealOrbDetonate, heal_effects.1);
}

fn create_ice_storm_particles(mut effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let mut ice_colour_grad = Gradient::new();
    ice_colour_grad.add_key(0.0, Vec4::new(0.0, 25.0, 180.0, 255.0));
    ice_colour_grad.add_key(0.5, Vec4::new(255.0, 255.0, 255.0, 255.0));
    ice_colour_grad.add_key(1.0, Vec4::new(0.0, 25.0, 180.0, 0.0));

    let mut ice_size_grad = Gradient::new();
    ice_size_grad.add_key(0.0, Vec2::new(1.0, 1.0));
    ice_size_grad.add_key(1.0, Vec2::new(0.0, 0.0));

    let writer_ice = ExprWriter::new();

    let age_ice = writer_ice.lit(0.).expr();
    let init_age_ice = SetAttributeModifier::new(Attribute::AGE, age_ice);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice = writer_ice.lit(0.6).uniform(writer_ice.lit(1.3)).expr();
    let init_lifetime_ice = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice);

    let init_pos_ice = SetPositionCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       radius: writer_ice.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice = SetVelocityCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       speed: writer_ice.lit(0.1).expr(),
    };

    let init_ang_vel_ice = SetVelocityTangentModifier {
        axis: writer_ice.lit(Vec3::Z).expr(),
        speed: writer_ice.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice.lit(Vec3::ZERO).expr()
    };

    let mut module_ice = writer_ice.finish();

    let ice_effect = effects.add(
        EffectAsset::new(200, Spawner::rate(30.0.into()), module_ice)
            .with_name("Ice Storm")
            .init(init_pos_ice)
            .init(init_vel_ice)
            .init(init_lifetime_ice)
            .init(init_age_ice)
            .init(init_ang_vel_ice)
            .render(ColorOverLifetimeModifier {
                gradient: ice_colour_grad.clone(),
            })
            .render(SizeOverLifetimeModifier {
                gradient: ice_size_grad.clone(),
                screen_space_size: false
            })
    );

    let writer_ice_end = ExprWriter::new();

    let age_ice_end = writer_ice_end.lit(0.).expr();
    let init_age_ice_end = SetAttributeModifier::new(Attribute::AGE, age_ice_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice_end = writer_ice_end.lit(0.1).expr();
    let init_lifetime_ice_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice_end);

    let init_pos_ice_end = SetPositionCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       radius: writer_ice_end.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice_end = SetVelocityCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       speed: writer_ice_end.lit(10.0).expr(),
    };

    let init_ang_vel_ice_end = SetVelocityTangentModifier {
        axis: writer_ice_end.lit(Vec3::Z).expr(),
        speed: writer_ice_end.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice_end.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice_end.lit(Vec3::ZERO).expr()
    };

    let mut module_ice_end = writer_ice_end.finish();

    let ice_end_effect = effects.add(
        EffectAsset::new(500, Spawner::rate(60.0.into()), module_ice_end)
            .with_name("Ice Storm Finish")
            .init(init_pos_ice_end)
            .init(init_vel_ice_end)
            .init(init_lifetime_ice_end)
            .init(init_age_ice_end)
            .init(init_ang_vel_ice_end)
            .render(ColorOverLifetimeModifier { gradient: ice_colour_grad.clone() })
            .render(SizeOverLifetimeModifier { gradient: ice_size_grad.clone(), screen_space_size: false })
    );

    return (ice_effect, ice_end_effect);
}

fn create_fire_ball_particles(mut effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let mut ice_colour_grad = Gradient::new();
    ice_colour_grad.add_key(0.0, Vec4::new(0.0, 25.0, 180.0, 255.0));
    ice_colour_grad.add_key(0.5, Vec4::new(255.0, 255.0, 255.0, 255.0));
    ice_colour_grad.add_key(1.0, Vec4::new(0.0, 25.0, 180.0, 0.0));

    let mut ice_size_grad = Gradient::new();
    ice_size_grad.add_key(0.0, Vec2::new(1.0, 1.0));
    ice_size_grad.add_key(1.0, Vec2::new(0.0, 0.0));

    let writer_ice = ExprWriter::new();

    let age_ice = writer_ice.lit(0.).expr();
    let init_age_ice = SetAttributeModifier::new(Attribute::AGE, age_ice);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice = writer_ice.lit(0.6).uniform(writer_ice.lit(1.3)).expr();
    let init_lifetime_ice = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice);

    let init_pos_ice = SetPositionCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       radius: writer_ice.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice = SetVelocityCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       speed: writer_ice.lit(0.1).expr(),
    };

    let init_ang_vel_ice = SetVelocityTangentModifier {
        axis: writer_ice.lit(Vec3::Z).expr(),
        speed: writer_ice.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice.lit(Vec3::ZERO).expr()
    };

    let mut module_ice = writer_ice.finish();

    let ice_effect = effects.add(
        EffectAsset::new(200, Spawner::rate(30.0.into()), module_ice)
            .with_name("Ice Storm")
            .init(init_pos_ice)
            .init(init_vel_ice)
            .init(init_lifetime_ice)
            .init(init_age_ice)
            .init(init_ang_vel_ice)
            .render(ColorOverLifetimeModifier {
                gradient: ice_colour_grad.clone(),
            })
            .render(SizeOverLifetimeModifier {
                gradient: ice_size_grad.clone(),
                screen_space_size: false
            })
    );

    let writer_ice_end = ExprWriter::new();

    let age_ice_end = writer_ice_end.lit(0.).expr();
    let init_age_ice_end = SetAttributeModifier::new(Attribute::AGE, age_ice_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice_end = writer_ice_end.lit(0.1).expr();
    let init_lifetime_ice_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice_end);

    let init_pos_ice_end = SetPositionCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       radius: writer_ice_end.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice_end = SetVelocityCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       speed: writer_ice_end.lit(10.0).expr(),
    };

    let init_ang_vel_ice_end = SetVelocityTangentModifier {
        axis: writer_ice_end.lit(Vec3::Z).expr(),
        speed: writer_ice_end.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice_end.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice_end.lit(Vec3::ZERO).expr()
    };

    let module_ice_end = writer_ice_end.finish();

    let ice_end_effect = effects.add(
        EffectAsset::new(500, Spawner::rate(60.0.into()), module_ice_end)
            .with_name("Ice Storm Finish")
            .init(init_pos_ice_end)
            .init(init_vel_ice_end)
            .init(init_lifetime_ice_end)
            .init(init_age_ice_end)
            .init(init_ang_vel_ice_end)
            .render(ColorOverLifetimeModifier { gradient: ice_colour_grad })
            .render(SizeOverLifetimeModifier { gradient: ice_size_grad, screen_space_size: false })
    );

    return (ice_effect, ice_end_effect);
}

fn create_heal_orb_particles(mut effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let mut ice_colour_grad = Gradient::new();
    ice_colour_grad.add_key(0.0, Vec4::new(0.0, 25.0, 180.0, 255.0));
    ice_colour_grad.add_key(0.5, Vec4::new(255.0, 255.0, 255.0, 255.0));
    ice_colour_grad.add_key(1.0, Vec4::new(0.0, 25.0, 180.0, 0.0));

    let mut ice_size_grad = Gradient::new();
    ice_size_grad.add_key(0.0, Vec2::new(1.0, 1.0));
    ice_size_grad.add_key(1.0, Vec2::new(0.0, 0.0));

    let writer_ice = ExprWriter::new();

    let age_ice = writer_ice.lit(0.).expr();
    let init_age_ice = SetAttributeModifier::new(Attribute::AGE, age_ice);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice = writer_ice.lit(0.6).uniform(writer_ice.lit(1.3)).expr();
    let init_lifetime_ice = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice);

    let init_pos_ice = SetPositionCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       radius: writer_ice.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice = SetVelocityCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       speed: writer_ice.lit(0.1).expr(),
    };

    let init_ang_vel_ice = SetVelocityTangentModifier {
        axis: writer_ice.lit(Vec3::Z).expr(),
        speed: writer_ice.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice.lit(Vec3::ZERO).expr()
    };

    let mut module_ice = writer_ice.finish();

    let ice_effect = effects.add(
        EffectAsset::new(200, Spawner::rate(30.0.into()), module_ice)
            .with_name("Ice Storm")
            .init(init_pos_ice)
            .init(init_vel_ice)
            .init(init_lifetime_ice)
            .init(init_age_ice)
            .init(init_ang_vel_ice)
            .render(ColorOverLifetimeModifier {
                gradient: ice_colour_grad.clone(),
            })
            .render(SizeOverLifetimeModifier {
                gradient: ice_size_grad.clone(),
                screen_space_size: false
            })
    );

    let writer_ice_end = ExprWriter::new();

    let age_ice_end = writer_ice_end.lit(0.).expr();
    let init_age_ice_end = SetAttributeModifier::new(Attribute::AGE, age_ice_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice_end = writer_ice_end.lit(0.1).expr();
    let init_lifetime_ice_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice_end);

    let init_pos_ice_end = SetPositionCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       radius: writer_ice_end.lit(0.05).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice_end = SetVelocityCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       speed: writer_ice_end.lit(10.0).expr(),
    };

    let init_ang_vel_ice_end = SetVelocityTangentModifier {
        axis: writer_ice_end.lit(Vec3::Z).expr(),
        speed: writer_ice_end.lit(-5.0 * std::f32::consts::PI).uniform(writer_ice_end.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice_end.lit(Vec3::ZERO).expr()
    };

    let mut module_ice_end = writer_ice_end.finish();

    let ice_end_effect = effects.add(
        EffectAsset::new(500, Spawner::rate(60.0.into()), module_ice_end)
            .with_name("Ice Storm Finish")
            .init(init_pos_ice_end)
            .init(init_vel_ice_end)
            .init(init_lifetime_ice_end)
            .init(init_age_ice_end)
            .init(init_ang_vel_ice_end)
            .render(ColorOverLifetimeModifier { gradient: ice_colour_grad })
            .render(SizeOverLifetimeModifier { gradient: ice_size_grad, screen_space_size: false })
    );

    return (ice_effect, ice_end_effect);
}
