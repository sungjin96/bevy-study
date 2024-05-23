use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateConfig};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::entity::player::player_assets_collection::*;
use crate::entity::AnimationTimer;
use crate::entity::player::player_stop_animation::*;
use crate::entity::player::player_walking_animation::*;


pub struct PlayerAnimationPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum PlayerState {
    #[default]
    Loading,
    Complete,
    StopByFront,
    StopByBack,
    StopByLeft,
    StopByRight,
    WalkingByFront,
    WalkingByBack,
    WalkingByLeft,
    WalkingByRight,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_loading_state(
                LoadingState::new(PlayerState::Loading)
                    .continue_to_state(PlayerState::Complete)
            )
            .configure_loading_state(LoadingStateConfig::new(PlayerState::Loading).load_collection::<PlayerAssetCollection>())
            .add_systems(OnEnter(PlayerState::Complete), spawn_player)
            .add_systems(Update, (
                update_player_stop_by_front.run_if(in_state(PlayerState::Complete)),
                update_player_stop_by_front.run_if(in_state(PlayerState::StopByFront)),
                update_player_stop_by_back.run_if(in_state(PlayerState::StopByBack)),
                update_player_stop_by_left.run_if(in_state(PlayerState::StopByLeft)),
                update_player_stop_by_right.run_if(in_state(PlayerState::StopByRight)),
                update_player_walking_by_front.run_if(in_state(PlayerState::WalkingByFront)),
                update_player_walking_by_back.run_if(in_state(PlayerState::WalkingByBack)),
                update_player_walking_by_left.run_if(in_state(PlayerState::WalkingByLeft)),
                update_player_walking_by_right.run_if(in_state(PlayerState::WalkingByRight))
            ));
    }
}

fn spawn_player(mut commands: Commands, asset: Res<PlayerAssetCollection>) {
    commands.spawn((
        SpriteBundle {
            texture: asset.stop_by_front.clone(),
            ..default()
        },
        TextureAtlas::from(asset.walking_by_front_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 1.0
        },
        Name::new("Player")
    ));
}

