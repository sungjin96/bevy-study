use bevy::prelude::*;

mod player;

use player::PlayerPlugin;

pub struct EntityPlugin;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}

