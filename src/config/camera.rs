use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanCamPlugin::default())
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 512.0,
        min_height: 288.0,
    };

   commands.spawn(camera)
       .insert(PanCam {
           grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
           enabled: true, // when false, controls are disabled. See toggle example.
           zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
           min_scale: 1., // prevent the camera from zooming too far in
           max_scale: Some(40.), // prevent the camera from zooming too far out
           ..default()
       });
}