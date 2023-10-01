use std::ops::Deref;
use bevy::prelude::{Query, Res};
use crate::components::{GameTick, GameTime, Ship, ShipItem};
use bevy::{log};

pub fn ship_items_system(
    mut ship_query : Query<&mut Ship>,
    time : Res<GameTime>,
    // get tick

){
    let mut repair_center_grids = Vec::new();
    //let mut cannon_shots = Vec::new();

    let ticks_per_second = 60;
    let game_tick = time.tick;



    ship_query.iter_mut().for_each(|mut ship| {
        ship.items.iter_mut().for_each(|item| {
            match item.item {
                ShipItem::AmmoBox(_) => {}
                ShipItem::FuelTank(_) => {}
                ShipItem::Cannon(cannon) => {
                    let ticks_per_shot = (ticks_per_second as f32 / cannon.shot_cooldown_seconds()) as u32;
                    if cannon.active && game_tick > (cannon.last_shot_tick + ticks_per_shot) {
                        //cannon_shots.push((item.pos, item.rotation));
                        log::info!("cannon shot");
                    }
                }
                ShipItem::Thruster(_) => {}
                ShipItem::RepairModule => {
                    repair_center_grids.push((item.pos));
                }
            }
        });
    });
}



/*
pub fn ship_movement_system(){
    log::info!("ship_movement_system");
}
pub fn ship_shoot_system(){
    log::info!("ship_movement_system");
}
pub fn enemy_die_system(){
    log::info!("ship_movement_system");
}
pub fn projectile_system(){
    log::info!("ship_movement_system");
}
*/