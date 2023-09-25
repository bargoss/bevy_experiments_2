use bevy::DefaultPlugins;
use bevy::prelude::App;

mod systems;
mod components;
mod utilities;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
