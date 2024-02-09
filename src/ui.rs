use bevy::{prelude::*, winit::WinitSettings};
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(600.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            });
        });
}
