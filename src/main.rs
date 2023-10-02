use bevy::{DefaultPlugins, log};
use bevy::ecs::schedule::{ScheduleLabel, SystemSetConfig};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{App, FixedUpdate, IntoSystemSetConfig, IntoSystemSetConfigs, Schedule, SystemSet};
use crate::components::{GameTick, GameTime};
use crate::systems::{increment_tick};
//use crate::systems::{enemy_die_system, projectile_system, ship_movement_system, ship_shoot_system};
use bevy::prelude::IntoSystemConfigs;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

mod systems;
mod components;
mod utilities;
mod games;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreFixedUpdate;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreFixedUpdateSystemsSet;
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixedUpdateSystemsSet;
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostFixedUpdateSystemsSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(games::ship_fight::Config)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(GameTime::default())
        .configure_set(FixedUpdate, PreFixedUpdateSystemsSet.before(FixedUpdateSystemsSet))
        .configure_set(FixedUpdate, FixedUpdateSystemsSet)
        .configure_set(FixedUpdate, PostFixedUpdateSystemsSet.after(FixedUpdateSystemsSet))

        .add_systems(FixedUpdate,(
            (increment_tick).in_set(PreFixedUpdateSystemsSet),
            //(ship_movement_system, ship_shoot_system, enemy_die_system, projectile_system).in_set(FixedUpdateSystemsSet),
        ))
        .run();
}

fn log_string(s: &str) {
    log::info!("{}", s);
}
