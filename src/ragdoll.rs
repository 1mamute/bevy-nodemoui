use bevy::{
    app::{App, Plugin, Update},
    asset::{Assets, Handle},
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt, Parent},
    log::info,
    math::Vec2,
    prelude::{default, SpatialBundle},
    render::{
        camera::Camera,
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::{GlobalTransform, Transform},
    window::{CursorMoved, PrimaryWindow, Window},
};

use crate::{AppState, MainCamera};

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RagdollSpawnEvent>();
        app.add_systems(OnEnter(AppState::Playback), ragdoll_setup)
            .add_systems(Update, ragdoll_spawn.run_if(in_state(AppState::Playback)));
        app.add_systems(OnExit(AppState::Playback), ragdoll_cleanup);

        app.init_resource::<MouseCoordinates>();
        app.add_systems(
            Update,
            (
                ragdoll_select_event_system.run_if(in_state(AppState::Playback)),
                cursor_position_to_world.run_if(in_state(AppState::Playback)),
            ),
        );
    }
}

#[derive(Event)]
pub struct RagdollSpawnEvent(pub Name);

fn ragdoll_setup(mut event_writer: EventWriter<RagdollSpawnEvent>) {
    info!("Setup ragdolls");
    event_writer.send(RagdollSpawnEvent(Name::new("Ragdoll 1")));
}

#[derive(Component)]
struct PlayerName(Name);

#[derive(Component)]
struct RagdollRadius {
    radius: f32,
}

#[derive(Component)]
struct RagdollBorder;

fn ragdoll_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<RagdollSpawnEvent>,
) {
    const RAGDOLL_RADIUS: f32 = 7.;

    for event in event_reader.read() {
        info!("Received RagdownSpawnEvent");

        // Spawn Circle
        let ragdoll = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(RAGDOLL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(-150.0, 0.0, 2.0),
                ..default()
            })
            .insert(event.0.clone())
            .insert(PlayerName(Name::new("f0rest")))
            .insert(RagdollRadius {
                radius: RAGDOLL_RADIUS,
            })
            .id();

        // Border circle
        // TODO: fix border blinking and appearing above ragdoll
        commands.entity(ragdoll).with_children(|parent| {
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(RAGDOLL_RADIUS * 1.15).into())
                        .into(), // Slightly larger than the ragdoll
                    material: materials.add(ColorMaterial::from(Color::NONE)), // Initially transparent
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                })
                .insert(SpatialBundle { ..default() })
                .insert(RagdollBorder);
        });
    }
}

fn ragdoll_cleanup(mut commands: Commands, query: Query<Entity, With<PlayerName>>) {
    info!("Cleanup ragdolls");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Event)]
pub struct RagdollSelectEvent(pub Entity);

fn ragdoll_select_event_system(
    ragdoll_query: Query<(Entity, &Transform, &RagdollRadius)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ragdoll_border_query: Query<(&mut Handle<ColorMaterial>, &Parent), With<RagdollBorder>>,
    mouse_coords: Res<MouseCoordinates>,
    // mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    // for event in mouse_button_input_events.read() {
    //     info!("{:?}", event);
    // }

    for event in cursor_moved_events.read() {
        let cursor_pos = mouse_coords.0;
        for (entity, transform, ragdoll_radius) in ragdoll_query.iter() {
            let ragdoll_pos = transform.translation.truncate(); // Vec3 to Vec2
            let distance = cursor_pos.distance(ragdoll_pos);

            let is_mouse_over = distance <= ragdoll_radius.radius;
            if let Some((mut border_material_handle, _)) = ragdoll_border_query
                .iter_mut()
                .find(|(_, parent)| parent.get() == entity)
            {
                let material = if is_mouse_over {
                    Color::RED // Muda para preto se o mouse estiver sobre
                } else {
                    Color::NONE // Muda para invisível ou cor do fundo quando o mouse não estiver sobre
                };
                *border_material_handle = materials.add(ColorMaterial::from(material));
            }
        }
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MouseCoordinates(Vec2);

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
