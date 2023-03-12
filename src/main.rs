

use bevy::{
    prelude::*m
    render::texture::ImageSettings,
    sprite:Anchor,
};

use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use floppy_corgi::pipes::{Pipe, PointsGate, SpawnPipe};

fn main(){
    App::new()
        .insert_resource(WindowDescriptor{
            title: "Floppy Corgi".to_string(),
            width: 1200.0,
            height: 600.0,
            ..Default::default()
        })
        .init_resource::<Score>()
        .insert_resource(ClearColor(Color::rgb(0.0, 42.0/255.0, 0.0)))
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .init_resource::<NumPipesToSpawn>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
            .continue_to_state(MyStates::Next)
            .with_collection::<MyAssets>(),
        )
        .add_state(MyStates::AssetLoading)
        .add_system_set(
            SystemSet::on_enter(MyStates::Next)
            .with_system(setup),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_system_set(SystemSet::on_update(MyStates::Next)
            .with_system(animate_sprite)
            .with_system(corgi_control)
            .with_system(align_to_window)
            .with_system(display_events)
            .with_system(despawn_pipes)
        )
        .run();
    }