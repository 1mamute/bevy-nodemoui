use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res},
    },
    gizmos::gizmos::Gizmos,
    hierarchy::BuildChildren,
    log::info,
    math::Vec3,
    prelude::default,
    render::{camera::Camera, color::Color},
    sprite::SpriteBundle,
    text::TextStyle,
    transform::components::{GlobalTransform, Transform},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignContent, AlignItems, AlignSelf, BorderColor, FlexDirection, JustifyContent, Style,
        UiRect, Val,
    },
    window::Window,
};

use crate::{maps::FloorPlant, AppState};

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playback), playback_setup)
            .add_systems(Update, draw_cursor.run_if(in_state(AppState::Playback)))
            .add_systems(
                Update,
                update_selected_map_on_event.run_if(in_state(AppState::Playback)),
            );
        //TODO: .add_systems(OnExit(AppState::MainMenu), playback_cleanup);

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
            border_color: BorderColor(Color::GREEN),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(1.0)),
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
                                        border_color: BorderColor(Color::RED),
                                        style: Style {
                                            align_self: AlignSelf::FlexStart,
                                            border: UiRect::all(Val::Px(1.0)),
                                            // justify_content: JustifyContent::FlexStart,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        // Map Floor Plant
                                        parent
                                            .spawn(NodeBundle {
                                                border_color: BorderColor(Color::RED),
                                                style: Style {
                                                    align_self: AlignSelf::FlexStart,
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    // justify_content: JustifyContent::FlexStart,
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
                                                border_color: BorderColor(Color::RED),
                                                style: Style {
                                                    align_self: AlignSelf::FlexStart,
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    // justify_content: JustifyContent::FlexStart,
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

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
