use bevy::prelude::ResMut;
use crate::components::{GameTick, GameTime};
use bevy::{log};

pub fn increment_tick(
    mut game_time : ResMut<GameTime>,
){
    let mut game_tick = &mut game_time.tick;
    *game_tick += 1;

    //bevy log
    log::info!("tick: {}", game_time.tick);
}