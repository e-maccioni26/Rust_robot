// src/main.rs
mod map;
mod robot;
mod simulation;
mod station;
mod gui;

use bevy::prelude::*;
use gui::{setup, update_map};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation EREEA".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update_map)
        .run();
}
