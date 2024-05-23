use bevy::prelude::*;

mod debug_plugin;
mod camera_plugin;

use debug_plugin::DebugPlugin;
use camera_plugin::CameraPlugin;

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
                CameraPlugin
            ));
    }
}