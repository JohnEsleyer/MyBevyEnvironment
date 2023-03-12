use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::*;
use player::*;
use common_components::*;




fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_system(detect_collisions)
        .run();

}

