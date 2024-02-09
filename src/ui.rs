use bevy::{prelude::*, winit::WinitSettings};

use crate::maps::FloorPlant;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, setup)
            .add_systems(PostStartup, button_map_renders)
            .add_systems(Update, button_system);
    }
}

const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.65, 0.65);
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.65, 0.65);
const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.65, 0.65);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn button_map_renders(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_query: Query<(&Name, With<FloorPlant>)>,
) {
    // Root UI Node for Map Buttons
    info!("running ");
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
        .with_children(|parent| {
            // For each map, create a button like this
            // Para cada MapBundle, criar um botão
            for (name, _) in map_query.iter() {
                info!("Found map: {:?}", name);
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

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                commands.entity(entity).despawn_recursive();
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
