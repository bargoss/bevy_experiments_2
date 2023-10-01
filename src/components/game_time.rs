//use bevy::ecs::component::Tick;
use bevy::prelude::{Component, Resource};
//use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// all the u32 traits
//#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,Default)]
//pub struct GameTick {
//    tick: u32
//}

// define GameTick as u32
pub type GameTick = u32;

#[derive(Resource, Default)]
pub struct GameTime{
    pub tick: GameTick,
}

// impl add u32
//impl Add<u32> for GameTick {
//    type Output = Self;
//
//    fn add(self, rhs: u32) -> Self::Output {
//        Self {
//            tick: self.tick + rhs,
//        }
//    }
//}

//impl Default for GameTick {
//    fn default() -> Self {
//        Self {
//            tick: 0,
//        }
//    }
//}





