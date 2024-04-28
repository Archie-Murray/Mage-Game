use bevy_hanabi::prelude::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ParticleType { FireBall, IceStorm, HealOrb, FireBallDetonate, IceStormFinish, HealOrbDetonate }

#[derive(Resource)]
pub struct AbilityParticles {
    pub particle_effects: HashMap<ParticleType, Handle<EffectAsset>>
}

impl Default for AbilityParticles {
    fn default() -> Self {
        return AbilityParticles {
            particle_effects: HashMap::new()
        };
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilityParticles>();
        app.add_systems(Startup, init_particles);
    }
}

pub fn init_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particles: ResMut<AbilityParticles>
) {
    let ( ice_effect,  ice_effects_end) = create_ice_storm_particles(&mut effects);
    let (fire_effect, fire_effects_end) = create_fire_ball_particles(&mut effects);
    let (heal_effect, heal_effects_end) = create_heal_orb_particles(&mut effects);
    particles.particle_effects.insert(ParticleType::IceStorm,         ice_effect);
    particles.particle_effects.insert(ParticleType::IceStormFinish,   ice_effects_end);
    particles.particle_effects.insert(ParticleType::FireBall,         fire_effect);
    particles.particle_effects.insert(ParticleType::FireBallDetonate, fire_effects_end);
    particles.particle_effects.insert(ParticleType::HealOrb,          heal_effect);
    particles.particle_effects.insert(ParticleType::HealOrbDetonate,  heal_effects_end);
}

fn create_ice_storm_particles(effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let mut ice_colour_grad = Gradient::new();
    // NOTE: Color is in format RGBA
    ice_colour_grad.add_key(0.0, Vec4::new(0.75, 0.9, 1.0, 1.0));
    ice_colour_grad.add_key(0.5, Vec4::new(1.0, 1.0, 1.0, 0.75));
    ice_colour_grad.add_key(1.0, Vec4::new(0.75, 0.9, 1.0, 0.0));

    let mut ice_size_grad = Gradient::new();
    ice_size_grad.add_key(0.0, Vec2::new(5.0, 5.0));
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
       radius: writer_ice.lit(5.0).uniform(writer_ice.lit(30.0)).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_ice = SetVelocityCircleModifier {
       center: writer_ice.lit(Vec3::ZERO).expr(),
       axis: writer_ice.lit(Vec3::Z).expr(),
       speed: writer_ice.lit(10.0).expr(),
    };

    let init_ang_vel_ice = SetVelocityTangentModifier {
        axis: writer_ice.lit(Vec3::X).expr(),
        speed: writer_ice.lit(0.0).uniform(writer_ice.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_ice.lit(Vec3::ZERO).expr()
    };

    let module_ice = writer_ice.finish();

    let ice_effect = effects.add(
        EffectAsset::new(2000, Spawner::rate(100.0.into()), module_ice)
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

    let age_ice_end = writer_ice_end.lit(0.0).expr();
    let init_age_ice_end = SetAttributeModifier::new(Attribute::AGE, age_ice_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_ice_end = writer_ice_end.lit(0.125).expr();
    let init_lifetime_ice_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_ice_end);

    let init_pos_ice_end = SetPositionCircleModifier {
       center: writer_ice_end.lit(Vec3::ZERO).expr(),
       axis: writer_ice_end.lit(Vec3::Z).expr(),
       radius: writer_ice_end.lit(10.0).expr(),
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
        EffectAsset::new(500, Spawner::once(60.0.into(), true), module_ice_end)
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

fn create_fire_ball_particles(effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let fire_colour_grad = Gradient::linear(
        Vec4::new(1.0, 1.0, 0.0, 1.0),
        Vec4::new(1.0, 0.0, 0.0, 0.5)
    );

    let fire_size_grad = Gradient::linear(Vec2::ZERO, Vec2::splat(10.0));

    let writer_fire = ExprWriter::new();

    let age_fire = writer_fire.lit(0.).expr();
    let init_age_fire = SetAttributeModifier::new(Attribute::AGE, age_fire);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_fire = writer_fire.lit(0.6).uniform(writer_fire.lit(1.3)).expr();
    let init_lifetime_fire = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_fire);

    let init_pos_fire = SetPositionCircleModifier {
       center: writer_fire.lit(Vec3::ZERO).expr(),
       axis: writer_fire.lit(Vec3::Z).expr(),
       radius: writer_fire.lit(2.0).uniform(writer_fire.lit(10.0)).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_fire = SetVelocityCircleModifier {
       center: writer_fire.lit(Vec3::ZERO).expr(),
       axis: writer_fire.lit(Vec3::Z).expr(),
       speed: writer_fire.lit(0.1).expr(),
    };

    let init_ang_vel_fire = SetVelocityTangentModifier {
        axis: writer_fire.lit(Vec3::Z).expr(),
        speed: writer_fire.lit(0.0).uniform(writer_fire.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_fire.lit(Vec3::ZERO).expr()
    };

    let module_fire = writer_fire.finish();

    let fire_effect = effects.add(
        EffectAsset::new(200, Spawner::rate(30.0.into()), module_fire)
            .with_name("fire Storm")
            .init(init_pos_fire)
            .init(init_vel_fire)
            .init(init_lifetime_fire)
            .init(init_age_fire)
            .init(init_ang_vel_fire)
            .render(ColorOverLifetimeModifier {
                gradient: fire_colour_grad.clone(),
            })
            .render(SizeOverLifetimeModifier {
                gradient: fire_size_grad.clone(),
                screen_space_size: false
            })
    );
    let fire_end_size = Gradient::linear(Vec2::splat(4.0), Vec2::ZERO);
    let writer_fire_end = ExprWriter::new();

    let age_fire_end = writer_fire_end.lit(0.).expr();
    let init_age_fire_end = SetAttributeModifier::new(Attribute::AGE, age_fire_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_fire_end = writer_fire_end.lit(0.1).expr();
    let init_lifetime_fire_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_fire_end);

    let init_pos_fire_end = SetPositionCircleModifier {
       center: writer_fire_end.lit(Vec3::ZERO).expr(),
       axis: writer_fire_end.lit(Vec3::Z).expr(),
       radius: writer_fire_end.lit(0.05).uniform(writer_fire_end.lit(1.0)).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_fire_end = SetVelocityCircleModifier {
       center: writer_fire_end.lit(Vec3::ZERO).expr(),
       axis: writer_fire_end.lit(Vec3::Z).expr(),
       speed: writer_fire_end.lit(10.0).expr(),
    };

    let init_ang_vel_fire_end = SetVelocityTangentModifier {
        axis: writer_fire_end.lit(Vec3::Z).expr(),
        speed: writer_fire_end.lit(-5.0 * std::f32::consts::PI).uniform(writer_fire_end.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_fire_end.lit(Vec3::ZERO).expr()
    };

    let module_fire_end = writer_fire_end.finish();

    let fire_end_effect = effects.add(
        EffectAsset::new(500, Spawner::once(30.0.into(), true), module_fire_end)
            .with_name("fire Storm Finish")
            .init(init_pos_fire_end)
            .init(init_vel_fire_end)
            .init(init_lifetime_fire_end)
            .init(init_age_fire_end)
            .init(init_ang_vel_fire_end)
            .render(ColorOverLifetimeModifier { gradient: fire_colour_grad })
            .render(SizeOverLifetimeModifier { gradient: fire_end_size, screen_space_size: false })
    );

    return (fire_effect, fire_end_effect);
}

fn create_heal_orb_particles(effects: &mut ResMut<Assets<EffectAsset>>) -> (Handle<EffectAsset>, Handle<EffectAsset>) {
    let heal_colour_grad = Gradient::linear(Vec4::new(1.0, 0.0, 0.0, 1.0), Vec4::new(1.0, 0.0, 0.0, 0.0));

    let heal_size_grad = Gradient::linear(Vec2::splat(4.0), Vec2::ZERO);

    let writer_heal = ExprWriter::new();

    let age_heal = writer_heal.lit(0.).expr();
    let init_age_heal = SetAttributeModifier::new(Attribute::AGE, age_heal);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_heal = writer_heal.lit(0.6).uniform(writer_heal.lit(1.3)).expr();
    let init_lifetime_heal = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_heal);

    let init_pos_heal = SetPositionCircleModifier {
       center: writer_heal.lit(Vec3::ZERO).expr(),
       axis: writer_heal.lit(Vec3::Z).expr(),
       radius: writer_heal.lit(10.0).uniform(writer_heal.lit(15.0)).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_heal = SetVelocityCircleModifier {
       center: writer_heal.lit(Vec3::ZERO).expr(),
       axis: writer_heal.lit(Vec3::Z).expr(),
       speed: writer_heal.lit(1.0).uniform(writer_heal.lit(30.0)).expr(),
    };

    let init_ang_vel_heal = SetVelocityTangentModifier {
        axis: writer_heal.lit(Vec3::Z).expr(),
        speed: writer_heal.lit(-5.0 * std::f32::consts::PI).uniform(writer_heal.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_heal.lit(Vec3::ZERO).expr()
    };

    let module_heal = writer_heal.finish();

    let heal_effect = effects.add(
        EffectAsset::new(400, Spawner::rate(40.0.into()), module_heal)
            .with_name("heal Storm")
            .init(init_pos_heal)
            .init(init_vel_heal)
            .init(init_lifetime_heal)
            .init(init_age_heal)
            .init(init_ang_vel_heal)
            .render(ColorOverLifetimeModifier {
                gradient: heal_colour_grad.clone(),
            })
            .render(SizeOverLifetimeModifier {
                gradient: heal_size_grad.clone(),
                screen_space_size: false
            })
    );

    let writer_heal_end = ExprWriter::new();

    let age_heal_end = writer_heal_end.lit(0.).expr();
    let init_age_heal_end = SetAttributeModifier::new(Attribute::AGE, age_heal_end);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime_heal_end = writer_heal_end.lit(0.1).expr();
    let init_lifetime_heal_end = SetAttributeModifier::new(Attribute::LIFETIME, lifetime_heal_end);

    let init_pos_heal_end = SetPositionCircleModifier {
       center: writer_heal_end.lit(Vec3::ZERO).expr(),
       axis: writer_heal_end.lit(Vec3::Z).expr(),
       radius: writer_heal_end.lit(0.05).uniform(writer_heal_end.lit(2.0)).expr(),
       dimension: ShapeDimension::Surface,
    };

    let init_vel_heal_end = SetVelocityCircleModifier {
       center: writer_heal_end.lit(Vec3::ZERO).expr(),
       axis: writer_heal_end.lit(Vec3::Z).expr(),
       speed: writer_heal_end.lit(10.0).expr(),
    };

    let init_ang_vel_heal_end = SetVelocityTangentModifier {
        axis: writer_heal_end.lit(Vec3::Z).expr(),
        speed: writer_heal_end.lit(-5.0 * std::f32::consts::PI).uniform(writer_heal_end.lit(5.0 * std::f32::consts::PI)).expr(),
        origin: writer_heal_end.lit(Vec3::ZERO).expr()
    };

    let module_heal_end = writer_heal_end.finish();

    let heal_end_effect = effects.add(
        EffectAsset::new(500, Spawner::once(20.0.into(), true), module_heal_end)
            .with_name("heal Storm Finish")
            .init(init_pos_heal_end)
            .init(init_vel_heal_end)
            .init(init_lifetime_heal_end)
            .init(init_age_heal_end)
            .init(init_ang_vel_heal_end)
            .render(ColorOverLifetimeModifier { gradient: heal_colour_grad })
            .render(SizeOverLifetimeModifier { gradient: heal_size_grad, screen_space_size: false })
    );

    return (heal_effect, heal_end_effect);
}
