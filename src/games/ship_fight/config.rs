use bevy::prelude::{App, FixedUpdate, Plugin, Startup};
use crate::FixedUpdateSystemsSet;
use crate::games::ship_fight::{init_game, ship_control, ship_input};
use bevy::prelude::IntoSystemConfigs;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_game);
        app.add_systems(FixedUpdate, (ship_input,ship_control).chain().in_set(FixedUpdateSystemsSet));
    }
}