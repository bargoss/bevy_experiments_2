use bevy::ecs::component::Tick;
use bevy::prelude::Component;
use derive_more::{Add, AddAssign};

#[derive(Component)]
pub struct Ship{
    pub items: Vec<ShipItemPlacement>,
}

impl Ship{
    pub fn new() -> Self{
        Self{
            items: vec![],
        }
    }

    pub fn get_grid(&self, pos: GridPos) -> Option<(ShipItem, GridOccupationType)>{
        for item in self.items.iter(){
            for (item_pos, occupation) in item.item.get_occupations().iter(){
                if *item_pos == pos{
                    return Some((item.item, *occupation));
                }
            }
        }
        None
    }
    pub fn get_grid_mut(&mut self, pos: GridPos) -> Option<(&mut ShipItem, GridOccupationType)>{
        for item in self.items.iter_mut(){
            for (item_pos, occupation) in item.item.get_occupations().iter(){
                if *item_pos == pos{
                    return Some((&mut item.item, *occupation));
                }
            }
        }
        None
    }

    pub fn get_occupied_grids(&self) -> Vec<GridPos>{
        let mut grids = vec![];
        for item in self.items.iter(){
            let rotation = item.rotation;
            for (occupation_pos, _) in item.item.get_occupations().iter(){
                grids.push(item.pos + rotate(rotation, *occupation_pos));
            }
        }
        grids
    }
    pub fn can_place_item(&self, item: ShipItem, pos: GridPos, rotation : ItemRotation) -> bool{
        let occupied_grids = self.get_occupied_grids();
        for (item_pos, _) in item.get_occupations().iter(){
            let rotated_pos = rotate(rotation, *item_pos);
            if occupied_grids.contains(&(pos + rotated_pos)){
                return false;
            }
        }
        true
    }

    pub fn try_place_item(&mut self, item: ShipItem, pos: GridPos, rotation : ItemRotation) -> bool {
        if self.can_place_item(item, pos, rotation) {
            self.items.push(ShipItemPlacement {
                item,
                pos,
                rotation
            });
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone)]
pub enum ItemRotation{
    Up,
    Right,
    Down,
    Left,
}

pub fn rotate(rotation: ItemRotation, pos: GridPos) -> GridPos{
    match rotation{
        ItemRotation::Up => pos,
        ItemRotation::Right => GridPos(pos.1, -pos.0),
        ItemRotation::Down => GridPos(-pos.0, -pos.1),
        ItemRotation::Left => GridPos(-pos.1, pos.0),
    }
}

#[derive(Copy, Clone)]
pub struct ShipItemPlacement{
    pub item: ShipItem,
    pub pos: GridPos,
    pub rotation: ItemRotation,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Add, AddAssign)]
pub struct GridPos(pub i8, pub i8);

pub trait GridOccupations {
    fn get_occupations(&self) -> Vec<(GridPos, GridOccupationType)>;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GridOccupationType {
    Regular,
    Button,
    AmmoSocket,
    FuelTankSocket,
}
#[derive(Copy, Clone)]
pub enum ShipItem{
    AmmoBox(AmmoBox),
    FuelTank(FuelTank),
    Cannon(Cannon),
    Thruster(Thruster),
    RepairModule,
}

impl GridOccupations for ShipItem{
    fn get_occupations(&self) -> Vec<(GridPos, GridOccupationType)> {
        match self{
            ShipItem::AmmoBox(_) => vec![
                (GridPos(0, 0), GridOccupationType::Regular)
            ],
            ShipItem::Cannon(_) => vec![
                (GridPos(0, 2), GridOccupationType::Regular),
                (GridPos(0, 1), GridOccupationType::Regular),
                (GridPos(0, 0), GridOccupationType::Button),
                (GridPos(1, 1), GridOccupationType::AmmoSocket),
            ],
            ShipItem::Thruster(_) => vec![
                (GridPos(0, 2), GridOccupationType::Button),
                (GridPos(0, 1), GridOccupationType::FuelTankSocket),
                (GridPos(0, 0), GridOccupationType::Regular),
            ],
            ShipItem::RepairModule => vec![(GridPos(0, 0), GridOccupationType::Regular)],
            ShipItem::FuelTank(_) => vec![(GridPos(0, 0), GridOccupationType::Regular)],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Thruster{
    pub active: bool,
    pub activated_tick: Tick,
}
#[derive(Copy, Clone)]
pub struct Cannon{
    pub active: bool,
    pub last_shot_tick: Tick,
    pub ammo_box: Option<AmmoBox>,
}

#[derive(Copy, Clone)]
pub struct FuelTank{
    pub capacity: f32,
    pub current: f32,
}

#[derive(Copy, Clone)]
pub struct AmmoBox {
    pub capacity: usize,
    pub current: usize,
}

#[derive(Component)]
pub struct Bullet{
    pub damage: f32,
    pub life_timer: f32,
}