use bevy::prelude::Query;
use crate::components::{Ship, ShipItem};

pub fn ship_system(
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