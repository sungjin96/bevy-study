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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAnimationPlugin)
            .add_systems(Update, (character_movement, sync_player_camera))
            .register_type::<Player>();
    }
}

fn sync_player_camera (
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for (player, player_transform) in &players {
        let pos = player_transform.translation;

        for mut transform in &mut cameras {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}


fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if button_input.just_pressed(KeyCode::KeyW) {
            next_state.set(PlayerState::WalkingByBack);
        }
        if button_input.just_pressed(KeyCode::KeyS) {
            next_state.set(PlayerState::WalkingByFront);
        }
        if button_input.just_pressed(KeyCode::KeyD) {
            next_state.set(PlayerState::WalkingByRight);
        }
        if button_input.just_pressed(KeyCode::KeyA) {
            next_state.set(PlayerState::WalkingByLeft);
        }

        if button_input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if button_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        if button_input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        if button_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
    }
}

