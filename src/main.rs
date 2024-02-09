mod diagnostics;
mod maps;
mod ui;

use bevy::{app::App, DefaultPlugins};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use maps::MapPlugin;
use ui::UIPlugin;

use crate::diagnostics::DiagnosticsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    {}
    // this code is compiled only if debug assertions are disabled (debug mode)
    #[cfg(debug_assertions)]
    {
        app.add_plugins(DiagnosticsPlugin);
    }

    app.add_plugins(UIPlugin);
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(MapPlugin);

    app.run();
}
