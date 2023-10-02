use bevy::prelude::{App, FixedUpdate, Plugin, Startup};
use crate::FixedUpdateSystemsSet;
use crate::games::{init_one_ship, ship_control, ship_input};
use bevy::prelude::IntoSystemConfigs;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_one_ship);
        app.add_systems(FixedUpdate, (ship_input,ship_control).chain().in_set(FixedUpdateSystemsSet));
    }
}