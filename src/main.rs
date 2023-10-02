use bevy::{DefaultPlugins, log};
use bevy::ecs::schedule::{ScheduleLabel, SystemSetConfig};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{App, FixedUpdate, IntoSystemSetConfig, IntoSystemSetConfigs, Schedule, SystemSet};
use crate::components::{GameTick, GameTime};
use crate::systems::{increment_tick};
//use crate::systems::{enemy_die_system, projectile_system, ship_movement_system, ship_shoot_system};
use bevy::prelude::IntoSystemConfigs;

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

// investigate this later:
//#[derive(SystemParam)]
//pub struct WhatIsSystemParam;

/*
let mut main_schedule = Schedule::new();
        main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
*/

// schedule.add_systems((propagate_system, modify_system).chain());

fn main() {
    //let pre_fixed_update_schedule = Schedule::default().add_systems((increment_tick, ship_movement_system, ship_shoot_system, enemy_die_system, projectile_system));
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameTime::default())
        .configure_set(FixedUpdate, PreFixedUpdateSystemsSet.before(FixedUpdateSystemsSet))
        .configure_set(FixedUpdate, FixedUpdateSystemsSet)
        .configure_set(FixedUpdate, PostFixedUpdateSystemsSet.after(FixedUpdateSystemsSet))

        //.configure_sets(FixedUpdate,PreFixedUpdateSystemsSet)

        .add_systems(FixedUpdate,(
            (increment_tick).in_set(PreFixedUpdateSystemsSet),
            //(ship_movement_system, ship_shoot_system, enemy_die_system, projectile_system).in_set(FixedUpdateSystemsSet),
        ))


        //.add_systems(FixedUpdate, (ship_movement_system, ship_shoot_system, enemy_die_system, projectile_system).chain())

    //ship_movement_system, ship_shoot_system, enemy_die_system, projectile_system
                // And run after increment_tick

        .run();
}

fn log_string(s: &str) {
    log::info!("{}", s);
}
