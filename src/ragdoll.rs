use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    core::Name,
    ecs::{
        event::{Event, EventReader, EventWriter},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, ResMut},
    },
    log::info,
    math::Vec3,
    prelude::default,
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

use crate::AppState;

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RagdollSpawnEvent>();
        app.add_systems(OnEnter(AppState::Playback), ragdoll_setup)
            .add_systems(Update, ragdoll_spawn.run_if(in_state(AppState::Playback)));
        //TODO: .add_systems(OnExit(AppState::MainMenu), ragdoll_cleanup);
    }
}

#[derive(Event)]
pub struct RagdollSpawnEvent(pub Name);

fn ragdoll_setup(mut event_writer: EventWriter<RagdollSpawnEvent>) {
    info!("Setup ragdolls");
    event_writer.send(RagdollSpawnEvent(Name::new("Ragdoll")));
}

fn ragdoll_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<RagdollSpawnEvent>,
) {
    for event in event_reader.read() {
        info!("Received RagdownSpawnEvent");
        // Spawn Circle
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            })
            .insert(event.0.clone());
    }
}
