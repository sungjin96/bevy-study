use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::log::info;
use bevy::prelude::{Commands, default, Entity, Query, Res, SpriteBundle, TextureAtlas, Timer, TimerMode, With};

use crate::entity::AnimationTimer;
use crate::entity::player::Player;
use crate::entity::player::player_assets_collection::PlayerAssetCollection;

pub fn spawn_player_stop_by_front(mut commands: Commands, asset: Res<PlayerAssetCollection>, mut player_query: Query<Entity, With<Player>>) {
    info!("Spawn!!");
    for player_entity in &mut player_query {
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).despawn();
        info!("플레이어 상태변화로 인한 Entity 삭제!");
    }

    commands.spawn((
        SpriteBundle {
            texture: asset.stop_by_front.clone(),
            ..default()
        },
        TextureAtlas::from(asset.stop_by_front_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 1.0
        },
        Name::new("Player")
    ));
}

pub fn spawn_player_stop_by_back(mut commands: Commands, asset: Res<PlayerAssetCollection>, mut player_query: Query<Entity, With<Player>>) {
    for player_entity in &mut player_query {
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).despawn();
        info!("플레이어 상태변화로 인한 Entity 삭제!");
    }

    commands.spawn((
        SpriteBundle {
            texture: asset.stop_by_back.clone(),
            ..default()
        },
        TextureAtlas::from(asset.stop_by_back_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 1.0
        },
        Name::new("Player")
    ));
}


pub fn spawn_player_stop_by_left(mut commands: Commands, asset: Res<PlayerAssetCollection>, mut player_query: Query<Entity, With<Player>>) {
    for player_entity in &mut player_query {
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).despawn();
        info!("플레이어 상태변화로 인한 Entity 삭제!");
    }

    commands.spawn((
        SpriteBundle {
            texture: asset.stop_by_left.clone(),
            ..default()
        },
        TextureAtlas::from(asset.stop_by_left_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 1.0
        },
        Name::new("Player")
    ));
}


pub fn spawn_player_stop_by_right(mut commands: Commands, asset: Res<PlayerAssetCollection>, mut player_query: Query<Entity, With<Player>>) {
    for player_entity in &mut player_query {
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).despawn();
        info!("플레이어 상태변화로 인한 Entity 삭제!");
    }

    commands.spawn((
        SpriteBundle {
            texture: asset.stop_by_right.clone(),
            ..default()
        },
        TextureAtlas::from(asset.stop_by_right_layout.clone()),
        AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
        },
        Player {
            speed: 1.0
        },
        Name::new("Player")
    ));
}