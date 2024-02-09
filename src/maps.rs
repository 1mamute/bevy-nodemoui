use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Handle},
    core::Name,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Res},
    },
    render::texture::Image,
};

const MAP_NAMES: [&str; 6] = [
    "Cache", "Dust2", "Inferno", "Mirage", // Nuke,
    "Overpass", "Train", // Vertigo,
];

#[derive(Component, Debug)]
struct FloorPlant(Handle<Image>);

#[derive(Bundle)]
struct MapBundle {
    name: Name,
    floor_plant: FloorPlant,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for map in MAP_NAMES {
        println!("Found {:?}", map);
        let floor_plant_handle: Handle<Image> =
            asset_server.load(format!("maps/de_{}_radar.png", map.to_lowercase()));
        commands.spawn(MapBundle {
            name: Name::new(map),
            floor_plant: FloorPlant(floor_plant_handle),
        });
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
