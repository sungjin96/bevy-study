use bevy::prelude::*;

mod debug;
mod camera;
mod entitiles;
mod common;
mod camera_movement;

use debug::DebugPlugin;
use camera::CameraPlugin;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "테스트 게임".into(),
                        resolution: (1280.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
            .add_plugins((
                DebugPlugin,
                CameraPlugin,
            ));
    }
}