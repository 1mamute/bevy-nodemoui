use bevy::{
    app::{App, Plugin, Startup},
    core_pipeline::{clear_color::ClearColor, core_2d::Camera2dBundle},
    ecs::system::Commands,
    render::color::Color,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
            .add_systems(Startup, setup);
    }
}
