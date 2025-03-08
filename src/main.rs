// src/main.rs
mod map;
mod robot;
mod simulation;
mod station;
mod gui;

use bevy::prelude::*;
use gui::{setup_map, spawn_robots, move_robots};

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
        // On lance d'abord la création de la carte
        .add_systems(Startup, setup_map)
        // Puis on spawn nos robots
        .add_systems(Startup, spawn_robots)
        // À chaque frame, on déplace nos robots
        .add_systems(Update, move_robots)
        .run();
}
