use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Default)]
pub struct Ship{
    // inputs:
    pub right_input: bool,
    pub left_input: bool,
    pub up_input: bool,
    pub down_input: bool,
    pub shoot_input: bool,
}

#[derive(Component, Clone, Copy, Default)]
pub struct LocalPlayer;

