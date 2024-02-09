use bevy::{
    app::{App, Plugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

#[cfg(debug_assertions)] // debug/dev builds only
pub struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin);
    }
}
