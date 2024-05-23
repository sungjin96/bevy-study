use bevy::prelude::*;

mod config;
mod entity;

use config::ConfigPlugin;
use entity::EntityPlugin;

fn main() {
    App::new()
        .add_plugins((
            ConfigPlugin,
            EntityPlugin
        ))
        .run();
}