mod player_assets_collection;
mod player_animation_plugin;
mod player_walking_animation;
mod player_stop_animation;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use player_animation_plugin::PlayerAnimationPlugin;
use player_animation_plugin::Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAnimationPlugin)
            .register_type::<Player>();
    }
}