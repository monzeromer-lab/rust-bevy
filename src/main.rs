use std::*;
use bevy::prelude::*;
fn sup() {
    println!("Hi");
}
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, sup)
        .run();
}
