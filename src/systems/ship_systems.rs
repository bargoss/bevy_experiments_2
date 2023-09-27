use bevy::prelude::Query;
use crate::components::{Ship, ShipItem};
use bevy::{log};
/*
pub fn ship_items_system(
    mut ship_query : Query<&mut Ship>,
    // get tick

){
    let mut healer_grids = Vec::new();
    let mut cannon_shots = Vec::new();


    ship_query.iter_mut().for_each(|mut ship|{
        ship.items.iter_mut().for_each(|item|{
            match item.item {
                ShipItem::AmmoBox(_) => {}
                ShipItem::FuelTank(_) => {}
                ShipItem::Cannon(cannon) => {
                    if cannon.active && cannon.shot_cooldown_seconds()
                }
                ShipItem::Thruster(_) => {}
                ShipItem::RepairModule => {
                    healer_grids.push(item.pos);
                }
            }
        });
    });
}
*/

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