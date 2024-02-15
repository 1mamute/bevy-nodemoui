mod floor_plant;
mod ragdoll;

use bevy::{
    app::{App, Plugin, Update},
    core::Name,
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        event::Event,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Res, Resource},
    },
    log::info,
    prelude::default,
    ui::{
        node_bundles::NodeBundle, AlignContent, AlignItems, FlexDirection, JustifyContent, Style,
        Val,
    },
};

use crate::AppState;
pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseState>();
        app.add_systems(OnEnter(AppState::Playback), playback_setup);
        app.add_systems(
            Update,
            check_mouse_state_changed.run_if(in_state(AppState::Playback)),
        );
        //TODO: .add_systems(OnExit(AppState::MainMenu), playback_cleanup);
        app.add_plugins(ragdoll::RagdollPlugin);
        app.add_plugins(floor_plant::FloorPlantPlugin);
        app.add_event::<MapSelectEvent>();
    }
}

#[derive(Component)]
pub struct RootPlaybackNode;

#[derive(Event)]
pub struct MapSelectEvent(pub Name);

fn playback_setup(mut commands: Commands) {
    info!("Entering AppState::Playback");
    // Root UI Node for Playback
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(RootPlaybackNode);
}

#[derive(Default, Resource)]
pub struct MouseState {
    pub over_entity: Option<Entity>,
}

fn check_mouse_state_changed(mouse_state: Res<MouseState>) {
    if mouse_state.is_changed() {
        info!("Mouse state changed: {:?}", mouse_state.over_entity);
    }
}
