mod player_assets_collection;
mod player_animation_plugin;
mod player_walking_animation;
mod player_stop_animation;
mod player_spawn_by_stop;
mod player_spawn_by_walking;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use player_animation_plugin::PlayerAnimationPlugin;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub(crate) speed: f32,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAnimationPlugin)
            .register_type::<Player>();
    }
}