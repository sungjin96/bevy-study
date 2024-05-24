use bevy::core::Name;
use bevy::prelude::{Commands, default, Entity, Query, Res, SpriteBundle, TextureAtlas, Timer, TimerMode, With};

use crate::entity::AnimationTimer;
use crate::entity::player::Player;
use crate::entity::player::player_assets_collection::PlayerAssetCollection;

pub fn spawn_player_walking(mut commands: Commands, asset: Res<PlayerAssetCollection>, mut player_query: Query<Entity, With<Player>>) {
    commands.spawn((
        SpriteBundle {
            texture: asset.walking.clone(),
            ..default()
        },
        TextureAtlas::from(asset.walking_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 40.0
        },
        Name::new("Player")
    ));
}

