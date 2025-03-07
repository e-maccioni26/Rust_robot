// src/gui.rs
use bevy::prelude::*;
use crate::map::{Map, Cell};

#[derive(Resource)]
pub struct SimulationMap {
    pub map: Map,
}

#[derive(Component)]
pub struct MapCell;

pub fn setup(mut commands: Commands) {
    // Création d'une caméra 2D
    commands.spawn(Camera2dBundle::default());

    // Création de la carte avec des dimensions et une seed fixes
    let map = Map::new(20, 10, 42);
    let cell_size: f32 = 20.0;

    // Affichage de chaque cellule
    for (y, row) in map.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Empty => Color::rgb(0.9, 0.9, 0.9),
                Cell::Obstacle => Color::rgb(0.3, 0.3, 0.3),
                Cell::Energy => Color::rgb(1.0, 1.0, 0.0),
                Cell::Mineral => Color::rgb(0.7, 0.7, 0.7),
                Cell::ScientificInterest => Color::rgb(0.0, 1.0, 0.0),
            };

            // Calculer la position pour centrer la carte
            let pos_x = x as f32 * cell_size - (map.width as f32 * cell_size) / 2.0 + cell_size / 2.0;
            let pos_y = y as f32 * cell_size - (map.height as f32 * cell_size) / 2.0 + cell_size / 2.0;

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(MapCell);
        }
    }

    commands.insert_resource(SimulationMap { map });
}

pub fn update_map() {
    // Système pour mettre à jour l'affichage en cas de modification de la carte.
    // Pour l'instant, ce système est vide.
}
