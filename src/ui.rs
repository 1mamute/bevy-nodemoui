use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{Changed, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    log::info,
    prelude::default,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, Interaction, JustifyContent, Style, UiRect, Val,
    },
    winit::WinitSettings,
};

use crate::{maps::FloorPlant, playback::MapSelectEvent, AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::game())
            .add_systems(OnEnter(AppState::MainMenu), buttons_setup)
            .add_systems(Update, buttons_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), buttons_cleanup);
    }
}

const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.65, 0.65);
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.65, 0.65);
const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.65, 0.65);

#[derive(Component)]
struct RootUINode;

fn buttons_setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_query: Query<(&Name, With<FloorPlant>)>,
) {
    info!("Entering AppState::MainMenu and Creating UI buttons for maps");
    // Root UI Node for Map Buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(RootUINode)
        .with_children(|parent| {
            // For each map, create a button like this
            for (name, ()) in map_query.iter() {
                info!("Creating button for map: {:?}", name);
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(Name::new(format!("{}", name.as_str())))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            name.as_str(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            }
        });
}

fn buttons_system(
    mut event_writer: EventWriter<MapSelectEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Name),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, name) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                let selected_map_name = Name::new(name.as_str().to_string());
                info!("Selected {}", selected_map_name);
                //TODO: don't know if this is right because we are getting "there is no root_node_entity in the playback.rs resize window function"
                event_writer.send(MapSelectEvent(selected_map_name));
                next_state.set(AppState::Playback);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn buttons_cleanup(mut commands: Commands, mut root_node_query: Query<Entity, With<RootUINode>>) {
    info!("Leaving AppState::MainMenu and Despawning UI buttons for maps");
    for root_node in &mut root_node_query {
        commands.entity(root_node).despawn_recursive();
    }
}
