
use bevy::prelude::*;
use bevy::app::App;

fn hello_world(){
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}




fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .run();
}