use bevy::{prelude::*, winit::WinitSettings};

use crate::{maps::FloorPlant, AppState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::MainMenu), buttons_setup)
            .add_systems(Update, buttons_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), buttons_cleanup);
    }
}

const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.65, 0.65);
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.65, 0.65);
const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.65, 0.65);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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
            // Para cada MapBundle, criar um botão
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
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                commands.entity(entity).despawn_recursive();
                next_state.set(AppState::DemoPlayback);
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

fn buttons_cleanup(mut commands: Commands, mut buttons_query: Query<Entity, With<RootUINode>>) {
    info!("Leaving AppState::MainMenu and Despawning UI buttons for maps");
    for rootuinode in &mut buttons_query {
        commands.entity(rootuinode).despawn_recursive();
    }
}
