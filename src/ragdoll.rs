use bevy::{
    app::{App, Plugin, Update},
    asset::{Assets, Handle},
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt, Parent},
    log::info,
    prelude::{default, SpatialBundle},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

use crate::{mouse::MouseCoordinates, AppState};

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RagdollSpawnEvent>();
        app.add_systems(OnEnter(AppState::Playback), ragdoll_setup);
        app.add_systems(
            Update,
            (
                ragdoll_hover_system.run_if(in_state(AppState::Playback)),
                detect_mouse_over_entity.run_if(in_state(AppState::Playback)),
                ragdoll_spawn.run_if(in_state(AppState::Playback)),
            ),
        );
        app.add_systems(OnExit(AppState::Playback), ragdoll_cleanup);

        app.add_event::<RagdollHoverEvent>();
        app.init_resource::<MouseState>();
    }
}

#[derive(Event)]
pub struct RagdollSpawnEvent(pub Name);

fn ragdoll_setup(mut event_writer: EventWriter<RagdollSpawnEvent>) {
    info!("Setup ragdolls");
    event_writer.send(RagdollSpawnEvent(Name::new("Ragdoll 1")));
}

#[derive(Component)]
struct Player {
    name: Name,
}

#[derive(Component)]
struct RagdollRadius {
    radius: f32,
}

#[derive(Component)]
struct RagdollBorder;

const RAGDOLL_RADIUS: f32 = 7.;
const RAGDOLL_BORDER_COLOR: Color = Color::RED;

fn ragdoll_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<RagdollSpawnEvent>,
) {
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
            .insert(Player {
                name: Name::new("f0rest"),
            })
            .insert(RagdollRadius {
                radius: RAGDOLL_RADIUS,
            })
            .id();

        // Border circle
        // TODO: fix border blinking and appearing above ragdoll
        commands.entity(ragdoll).with_children(|parent| {
            parent
                // Necessary to "anchor" the border to the ragdoll circle
                // https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
                .spawn(SpatialBundle { ..default() })
                .insert(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(RAGDOLL_RADIUS * 1.15).into())
                        .into(), // Slightly larger than the ragdoll
                    material: materials.add(ColorMaterial::from(Color::NONE)), // Initially transparent
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                })
                .insert(RagdollBorder);
        });
    }
}

fn ragdoll_cleanup(mut commands: Commands, query: Query<Entity, With<Player>>) {
    info!("Cleanup ragdolls");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Default, Resource)]
struct MouseState {
    over_entity: Option<Entity>,
}

#[derive(Event)]
pub struct RagdollHoverEvent {
    hovered_entity: Option<Entity>,
}

fn ragdoll_hover_system(
    mut ragdoll_hover_event: EventReader<RagdollHoverEvent>,
    mut mouse_state: ResMut<MouseState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ragdoll_border_query: Query<(&mut Handle<ColorMaterial>, &Parent), With<RagdollBorder>>,
) {
    for event in ragdoll_hover_event.read() {
        // Hide the border of the previous entity, if there's any
        if let Some(old_entity) = mouse_state.over_entity {
            if let Some((border_material_handle, _)) = ragdoll_border_query
                .iter()
                .find(|(_, parent)| parent.get() == old_entity)
            {
                if let Some(material) = materials.get_mut(border_material_handle) {
                    material.color = Color::NONE;
                }
            }
        }

        // Paint the border of the new entity, if there's any
        if let Some(new_entity) = event.hovered_entity {
            if let Some((border_material_handle, _)) = ragdoll_border_query
                .iter()
                .find(|(_, parent)| parent.get() == new_entity)
            {
                if let Some(material) = materials.get_mut(border_material_handle) {
                    material.color = RAGDOLL_BORDER_COLOR;
                }
            }
        }

        // Update the mouse state anyway
        mouse_state.over_entity = event.hovered_entity;
    }
}

fn detect_mouse_over_entity(
    mouse_coords: Res<MouseCoordinates>,
    ragdoll_query: Query<(Entity, &Transform, &RagdollRadius)>,
    mouse_state: ResMut<MouseState>,
    mut event_writer: EventWriter<RagdollHoverEvent>,
) {
    let mut current_over_entity: Option<Entity> = None;

    for (entity, transform, ragdoll_radius) in ragdoll_query.iter() {
        let distance = mouse_coords.0.distance(transform.translation.truncate());

        if distance <= ragdoll_radius.radius {
            current_over_entity = Some(entity);
            break;
        }
    }

    if current_over_entity != mouse_state.over_entity {
        event_writer.send(RagdollHoverEvent {
            hovered_entity: current_over_entity,
        });
    }
}
