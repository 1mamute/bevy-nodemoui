mod camera;
mod diagnostics;

use bevy::{app::App, DefaultPlugins};

use camera::CameraPlugin;
use diagnostics::DiagnosticsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, CameraPlugin));

    // Adiciona os plugins de diagnóstico apenas em modo debug
    #[cfg(debug_assertions)]
    {
        app.add_plugins(DiagnosticsPlugin);
    }

    app.run();
}
