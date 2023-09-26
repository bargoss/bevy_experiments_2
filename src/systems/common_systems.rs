use bevy::prelude::ResMut;
use crate::components::GameTime;

pub fn increment_tick(
    mut game_time : ResMut<GameTime>,
){
    let prev_tick_val = game_time.tick.get();
    game_time.tick.set(prev_tick_val + 1);
}