use bevy::{
    app::{App, Plugin, Update},
    asset::{Handle},
    core::Name,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    log::{debug, info},
    math::Vec3,
    prelude::default,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    ui::{node_bundles::NodeBundle, AlignSelf, Style},
    window::WindowResized,
};

use crate::{
    maps::FloorPlant,
    playback::{MapSelectEvent, RootPlaybackNode},
    AppState,
};

pub struct FloorPlantPlugin;

impl Plugin for FloorPlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_floor_plant_on_map_select.run_if(in_state(AppState::Playback)),
                on_resize_window.run_if(in_state(AppState::Playback)),
            ),
        );
        app.add_systems(OnExit(AppState::Playback), floor_plant_cleanup);
    }
}

fn draw_floor_plant_on_map_select(
    mut event_reader: EventReader<MapSelectEvent>,
    mut commands: Commands,
    root_node_query: Query<Entity, With<RootPlaybackNode>>,
    map_query: Query<(&Name, &FloorPlant)>,
) {
    for event in event_reader.read() {
        info!("draw_floor_plant_on_map_select received");

        match root_node_query.get_single() {
            Ok(root_node_entity) => {
                commands
                    .entity(root_node_entity)
                    .with_children(|root_node| {
                        // Iterate over Map Entities
                        if let Some((map_name, map_floor_plant)) =
                            map_query.iter().find(|(map_name, _)| **map_name == event.0)
                        {
                            info!("Found {} entity, writing text", map_name.as_str());
                            // Playback UI Node
                            root_node
                                .spawn(NodeBundle {
                                    style: Style {
                                        align_self: AlignSelf::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(SpriteBundle {
                                    transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
                                    texture: map_floor_plant.handle.clone(),
                                    ..default()
                                });
                        }
                    });
            }
            Err(QuerySingleError::NoEntities(_)) => {
                println!("Error: There is no root_node_entity!");
            }
            Err(QuerySingleError::MultipleEntities(_)) => {
                println!("Error: There is more than one root_node_entity!");
            }
        };
    }
}

// This system shows how to respond to a window being resized.
// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_window(
    mut floor_plant_sprite_query: Query<&mut Transform, With<Handle<Image>>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    match floor_plant_sprite_query.get_single_mut() {
        Ok(mut floor_plant_transform) => {
            for resize in resize_reader.read() {
                // When resolution is being changed
                debug!(
                    "Resizing floor plant to {:?} x {:?}",
                    resize.height, resize.width
                );

                // Exemplo: suaImagem.scale(novaLargura / larguraImagemOriginal, novaAltura / alturaImagemOriginal);
                floor_plant_transform.scale.y = resize.height / 1024_f32;
                floor_plant_transform.scale.x = resize.height / 1024_f32;
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no root_node_entity!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one root_node_entity!");
        }
    };
}

fn floor_plant_cleanup(mut commands: Commands, query: Query<Entity, With<FloorPlant>>) {
    info!("Cleanup floor plant");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
