use bevy::prelude::*;

mod config;
mod entity;
mod map;

fn main() {
    App::new()
        .add_plugins((
            config::ConfigPlugin,
            entity::EntityPlugin,
        ))
        .run();
}