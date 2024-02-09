mod diagnostics;
mod maps;
mod playback;
mod ui;

use bevy::{app::App, ecs::schedule::States, DefaultPlugins};

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
