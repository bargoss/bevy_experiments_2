use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Default)]
pub struct Ship{
    // inputs:
    pub right_input: bool,
    pub left_input: bool,
    pub shoot_input: bool,
}

impl Ship {
    pub fn reset_inputs(&mut self) {
        self.right_input = false;
        self.left_input = false;
        self.shoot_input = false;
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct LocalPlayer;

