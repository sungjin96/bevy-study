use bevy::log::info;
use bevy::prelude::{Query, Res, TextureAtlas, Time};
use crate::entity::AnimationTimer;

pub fn update_player_walking_by_front(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // 24 ~ 29
            if(sprite.index >= 24 && sprite.index < 29) {
                sprite.index = sprite.index + 1;
            } else {
                sprite.index = 24
            }
            // sprite.index = (sprite.index + 24) % 6;
        }
    }
}

pub fn update_player_walking_by_back(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if(sprite.index >= 30 && sprite.index < 35) {
                sprite.index = sprite.index + 1;
            } else {
                sprite.index = 30
            }
        }
    }
}

pub fn update_player_walking_by_right(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if(sprite.index >= 36 && sprite.index < 41) {
                sprite.index = sprite.index + 1;
            } else {
                sprite.index = 36
            }
        }
    }
}

pub fn update_player_walking_by_left(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if(sprite.index >= 42 && sprite.index < 47) {
                sprite.index = sprite.index + 1;
            } else {
                sprite.index = 42
            }
        }
    }
}