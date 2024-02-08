use bevy::{
    app::{App, Plugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

#[cfg(debug_assertions)] // debug/dev builds only
pub struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }
}
