use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, ResMut, Resource},
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

use crate::{AppState, MainCamera};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseCoordinates>();
        app.add_systems(
            Update,
            (cursor_position_to_world.run_if(in_state(AppState::Playback)),),
        );
    }
}

#[derive(Resource, Default)]
pub struct MouseCoordinates(pub Vec2);

fn cursor_position_to_world(
    mut mouse_coords: ResMut<MouseCoordinates>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::get_single() is OK
    let (camera, camera_transform) = match q_camera.get_single() {
        Ok((camera, camera_transform)) => (camera, camera_transform),
        Err(QuerySingleError::NoEntities(_)) => {
            panic!("Error: There is no primary camera!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Error: There is more than one primary camera!");
        }
    };

    // There is only one primary window, so we can similarly get it from the query:
    let window = match q_window.get_single() {
        Ok(window) => window,
        Err(QuerySingleError::NoEntities(_)) => {
            panic!("Error: There is no primary window!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Error: There is more than one primary window!");
        }
    };

    // check if the cursor is inside the window and get its position then ask bevy to convert into world coordinates
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_coords.0 = world_position;
        // debug!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
