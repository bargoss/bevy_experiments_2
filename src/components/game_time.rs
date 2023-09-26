use bevy::ecs::component::Tick;
use bevy::prelude::{Component, Resource};

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameTime{
    pub tick: Tick
}