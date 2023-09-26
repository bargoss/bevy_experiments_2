use bevy::{DefaultPlugins, log};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{App, FixedUpdate, IntoSystemSetConfig, IntoSystemSetConfigs, Schedule, SystemSet};

mod systems;
mod components;
mod utilities;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreFixedUpdate;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreFixedUpdateSystemsSet;

// investigate this later:
//#[derive(SystemParam)]
//pub struct WhatIsSystemParam;

/*
let mut main_schedule = Schedule::new();
        main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_schedule(PreFixedUpdate, Schedule::default())
        //.add_system_set()
        //.add_systems(PreFixedUpdate, log_string.system())
        .add_schedule(PreFixedUpdate, Schedule::default())
        // .configure_set(UiSystem::Focus.in_base_set(CoreSet::PreUpdate))
        .configure_set(PreFixedUpdateSystemsSet.before(FixedUpdate.set))
        //.configure_sets((PreFixedUpdate, FixedUpdate).chain())
        .run();
}

fn log_string(s: &str) {
    log::info!("{}", s);
}
