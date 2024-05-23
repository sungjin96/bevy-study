use bevy::prelude::{Query, Res, TextureAtlas, Time};
use crate::entity::AnimationTimer;

pub fn update_player_stop_by_front(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index = (sprite.index + 1) % 6;
        }
    }
}

pub fn update_player_stop_by_back(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index = (sprite.index + 1) % 6;
        }
    }
}

pub fn update_player_stop_by_left(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index = (sprite.index + 1) % 6;
        }
    }
}

pub fn update_player_stop_by_right(
    mut sprites: Query<(&mut TextureAtlas, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in &mut sprites {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index = (sprite.index + 1) % 6;
        }
    }
}