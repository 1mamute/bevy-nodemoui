mod diagnostics;
mod ui;

use bevy::{app::App, DefaultPlugins};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use diagnostics::DiagnosticsPlugin;
use ui::UIPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, UIPlugin));
    app.add_plugins(WorldInspectorPlugin::new());

    // Only add diagnostics plugin in debug mode
    #[cfg(debug_assertions)]
    {
        app.add_plugins(DiagnosticsPlugin);
    }

    app.run();
}
