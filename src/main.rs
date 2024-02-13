mod diagnostics;
mod main_menu;
mod maps;
mod mouse;
mod playback;
mod ragdoll;

use bevy::{
    app::{App, PluginGroup, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, schedule::States, system::Commands},
    prelude::default,
    window::{PresentMode, Window, WindowPlugin},
    winit::WinitSettings,
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use main_menu::MainMenuPlugin;
use maps::MapPlugin;
use mouse::MousePlugin;
use playback::PlaybackPlugin;

use crate::diagnostics::DiagnosticsPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    Playback,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));
    app.insert_resource(WinitSettings::game());
    app.add_state::<AppState>();
    app.add_systems(Startup, setup);

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    {}
    // this code is compiled only if debug assertions are disabled (debug mode)
    #[cfg(debug_assertions)]
    {
        app.add_plugins(DiagnosticsPlugin);
    }

    app.add_plugins(MapPlugin);
    app.add_plugins(MainMenuPlugin);
    app.add_plugins(MousePlugin);
    app.add_plugins(PlaybackPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
