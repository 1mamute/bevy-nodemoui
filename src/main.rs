mod diagnostics;
mod maps;
mod playback;
mod ragdoll;
mod ui;

use bevy::{
    app::{App, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        schedule::States,
        system::{Commands},
    },
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use maps::MapPlugin;
use playback::PlaybackPlugin;
use ui::UIPlugin;

use crate::diagnostics::DiagnosticsPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    Playback,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
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
    app.add_plugins(UIPlugin);
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
