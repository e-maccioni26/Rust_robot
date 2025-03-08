// src/gui.rs
use bevy::prelude::*;
use crate::map::{Map, Cell};

/// Ressource qui stocke la carte de la simulation
#[derive(Resource)]
pub struct SimulationMap {
    pub map: Map,
}

/// Composant pour les cellules de la carte
#[derive(Component)]
pub struct MapCell;

/// Composant pour nos robots
#[derive(Component)]
pub struct RobotSprite {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

/// Système qui installe la scène de base (caméra + carte + ressource `SimulationMap`)
pub fn setup_map(mut commands: Commands) {
    // Caméra 2D
    commands.spawn(Camera2dBundle::default());

    // Création de la carte
    let map = Map::new(20, 10, 42);
    let cell_size: f32 = 20.0;

    for (y, row) in map.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Empty => Color::rgb(0.9, 0.9, 0.9),
                Cell::Obstacle => Color::rgb(0.3, 0.3, 0.3),
                Cell::Energy => Color::rgb(1.0, 1.0, 0.0),
                Cell::Mineral => Color::rgb(0.7, 0.7, 0.7),
                Cell::ScientificInterest => Color::rgb(0.0, 1.0, 0.0),
            };

            // Conversion coordonnées grille -> coordonnées Bevy (centrage)
            let pos_x = x as f32 * cell_size - (map.width as f32 * cell_size) / 2.0 + cell_size / 2.0;
            let pos_y = y as f32 * cell_size - (map.height as f32 * cell_size) / 2.0 + cell_size / 2.0;

            commands
                .spawn(SpriteBundle {
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

    // On stocke la carte dans une ressource
    commands.insert_resource(SimulationMap { map });
}

/// Système qui spawn un ou plusieurs robots dans la scène
pub fn spawn_robots(mut commands: Commands) {
    // Exemple : on va créer deux robots
    let robot_positions = vec![(2.0, 3.0), (5.0, 5.0)];
    let cell_size: f32 = 20.0;
    let mut next_id = 1;

    for (x, y) in robot_positions {
        // Conversion grille -> coordonnées Bevy
        let pos_x = x * cell_size - (20.0 * cell_size) / 2.0 + cell_size / 2.0;
        let pos_y = y * cell_size - (10.0 * cell_size) / 2.0 + cell_size / 2.0;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE, // Couleur du robot
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 1.0), // Z=1 pour passer "au-dessus" de la carte
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RobotSprite {
                id: next_id,
                x,
                y,
            });

        next_id += 1;
    }
}

/// Système qui déplace les robots
/// Ici, simple exemple : on les fait se déplacer vers la droite
pub fn move_robots(
    mut query: Query<(&mut RobotSprite, &mut Transform)>,
    time: Res<Time>,
) {
    // Vitesse de déplacement en cellules/seconde
    let speed = 1.0;
    let cell_size = 20.0;

    for (mut robot, mut transform) in query.iter_mut() {
        // On fait avancer le robot en x en fonction du temps écoulé
        robot.x += speed * time.delta_seconds();

        // Mise à jour de la position Bevy
        let pos_x = robot.x * cell_size - (20.0 * cell_size) / 2.0 + cell_size / 2.0;
        let pos_y = robot.y * cell_size - (10.0 * cell_size) / 2.0 + cell_size / 2.0;

        transform.translation.x = pos_x;
        transform.translation.y = pos_y;
    }
}
