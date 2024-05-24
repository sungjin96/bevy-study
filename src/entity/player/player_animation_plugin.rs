use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateConfig, LoadingStateSet};
use crate::entity::player::{Player, PlayerState};

use crate::entity::player::player_assets_collection::*;
use crate::entity::player::player_spawn_by_stop::*;
use crate::entity::player::player_spawn_by_walking::*;
use crate::entity::player::player_stop_animation::*;
use crate::entity::player::player_walking_animation::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_loading_state(
                LoadingState::new(PlayerState::Loading)
                    .continue_to_state(PlayerState::Complete)
            )
            .configure_loading_state(LoadingStateConfig::new(PlayerState::Loading).load_collection::<PlayerAssetCollection>())
            .add_systems(OnEnter(PlayerState::StopByFront), spawn_player_stop_by_front)
            .add_systems(OnEnter(PlayerState::StopByBack), spawn_player_stop_by_back)
            .add_systems(OnEnter(PlayerState::StopByLeft), spawn_player_stop_by_left)
            .add_systems(OnEnter(PlayerState::StopByRight), spawn_player_stop_by_right)
            .add_systems(OnEnter(PlayerState::Complete), spawn_player_walking)
            .add_systems(Update, (
                update_player_stop_by_front.run_if(in_state(PlayerState::StopByFront)),
                update_player_stop_by_back.run_if(in_state(PlayerState::StopByBack)),
                update_player_stop_by_left.run_if(in_state(PlayerState::StopByLeft)),
                update_player_stop_by_right.run_if(in_state(PlayerState::StopByRight)),
                update_player_walking_by_front.run_if(in_state(PlayerState::WalkingByFront)),
                update_player_walking_by_back.run_if(in_state(PlayerState::WalkingByBack)),
                update_player_walking_by_left.run_if(in_state(PlayerState::WalkingByLeft)),
                update_player_walking_by_right.run_if(in_state(PlayerState::WalkingByRight)),
            ));
    }
}





