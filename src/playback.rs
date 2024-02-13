use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Handle},
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    log::{debug, info},
    math::Vec3,
    prelude::default,
    render::{color::Color, texture::Image},
    sprite::SpriteBundle,
    text::TextStyle,
    transform::components::Transform,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, Style, Val,
    },
    window::WindowResized,
};

use crate::{maps::FloorPlant, ragdoll::RagdollPlugin, AppState};

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playback), playback_setup)
            .add_systems(
                Update,
                (
                    update_selected_map_on_event.run_if(in_state(AppState::Playback)),
                    on_resize_window.run_if(in_state(AppState::Playback)),
                ),
            );
        //TODO: .add_systems(OnExit(AppState::MainMenu), playback_cleanup);

        app.add_plugins(RagdollPlugin);
        app.add_event::<MapSelectEvent>();
    }
}

#[derive(Component)]
struct RootUINode;

#[derive(Event)]
pub struct MapSelectEvent(pub Name);

fn playback_setup(mut commands: Commands) {
    info!("Entering AppState::DemoPlayback");
    // Root UI Node for Map Buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(RootUINode);
}

fn update_selected_map_on_event(
    mut event_reader: EventReader<MapSelectEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_node_query: Query<Entity, With<RootUINode>>,
    map_query: Query<(&Name, &FloorPlant)>,
) {
    for event in event_reader.read() {
        info!("update_selected_map_on_event received");

        match root_node_query.get_single() {
            Ok(root_node_entity) => {
                commands
                    .entity(root_node_entity)
                    .with_children(|root_node| {
                        // Iterate over Map Entities
                        for (map_name, map_floor_plant) in map_query.iter() {
                            if map_name.eq(&event.0) {
                                info!("Found {} entity, writing text", map_name.as_str());
                                // Playback Root UI Node
                                root_node
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_self: AlignSelf::FlexStart,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Map Floor Plant
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    align_self: AlignSelf::FlexStart,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(SpriteBundle {
                                                transform: Transform::from_scale(Vec3::new(
                                                    1.0, 1.0, 1.0,
                                                )),
                                                texture: map_floor_plant.handle.clone(),
                                                ..default()
                                            });

                                        // Map Name
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    align_self: AlignSelf::FlexStart,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(TextBundle::from_section(
                                                // Map Name
                                                map_name.as_str(),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                    });
                            }
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
