use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{QuerySingleError, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    log::info,
    prelude::default,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignSelf, Style,
    },
};

use crate::AppState;

use super::{MapSelectEvent, RootPlaybackNode};

pub struct PlaybackUIPlugin;

impl Plugin for PlaybackUIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<RagdollSpawnEvent>();
        // app.add_systems(OnEnter(AppState::Playback), ragdoll_setup);
        app.add_systems(
            Update,
            draw_ui_on_map_select.run_if(in_state(AppState::Playback)),
        );
        // app.add_systems(OnExit(AppState::Playback), ragdoll_cleanup);
        //
        // app.add_event::<RagdollHoverEvent>();
    }
}

fn draw_ui_on_map_select(
    mut event_reader: EventReader<MapSelectEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    root_node_query: Query<Entity, With<RootPlaybackNode>>,
) {
    for event in event_reader.read() {
        info!("draw_ui_on_map_select received");

        match root_node_query.get_single() {
            Ok(root_node_entity) => {
                commands
                    .entity(root_node_entity)
                    .with_children(|root_node| {
                        // Playback UI Node
                        root_node
                            .spawn(NodeBundle {
                                style: Style {
                                    align_self: AlignSelf::FlexStart,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(TextBundle::from_section(
                                // Map Name
                                event.0.as_str(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
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
