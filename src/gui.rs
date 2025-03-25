use bevy::prelude::*;
use crate::map::{Map, Cell};

#[derive(Resource)]
pub struct SimulationMap {
    pub map: Map,
}

#[derive(Component)]
pub struct MapCell;

#[derive(Component)]
pub struct RobotSprite {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

pub fn setup_map(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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

    commands.insert_resource(SimulationMap { map });
}

pub fn spawn_robots(mut commands: Commands) {
    let robot_positions = vec![(2.0, 3.0), (5.0, 5.0)];
    let cell_size: f32 = 20.0;
    let mut next_id = 1;

    for (x, y) in robot_positions {
        let pos_x = x * cell_size - (20.0 * cell_size) / 2.0 + cell_size / 2.0;
        let pos_y = y * cell_size - (10.0 * cell_size) / 2.0 + cell_size / 2.0;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE, 
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 1.0), 
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

pub fn move_robots(
    mut query: Query<(&RobotSprite, &mut Transform)>,
    time: Res<Time>,
) {
    let cell_size = 20.0;
    for (robot, mut transform) in query.iter_mut() {
        let pos_x = robot.x * cell_size - (20.0 * cell_size) / 2.0 + cell_size / 2.0;
        let pos_y = robot.y * cell_size - (10.0 * cell_size) / 2.0 + cell_size / 2.0;
        transform.translation.x = pos_x;
        transform.translation.y = pos_y;
    }
}


pub fn robot_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut RobotSprite>,
) {
   
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    
    if direction != Vec2::ZERO {
        for mut robot in query.iter_mut() {
            robot.x += direction.x;
            robot.y += direction.y;
        }
    }
}   