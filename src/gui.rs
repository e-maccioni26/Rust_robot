use bevy::prelude::*;
use crate::map::{Map, Cell};

#[derive(Resource)]
pub struct SimulationMap {
    pub map: Map,
}

#[derive(Resource)]
pub struct SelectedRobot(pub u32);

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
    mut selected: ResMut<SelectedRobot>,
    mut query: Query<&mut RobotSprite>,
    sim_map: Res<SimulationMap>,
) {

    if keyboard_input.just_pressed(KeyCode::Key1) {
        selected.0 = 1;
        println!("Robot sélectionné = 1");
    }
    if keyboard_input.just_pressed(KeyCode::Key2) {
        selected.0 = 2;
        println!("Robot sélectionné = 2");
    }

    let mut dx = 0.0;
    let mut dy = 0.0;
    if keyboard_input.just_pressed(KeyCode::Up) {
        dy = 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        dy = -1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        dx = -1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        dx = 1.0;
    }

    
    if dx != 0.0 || dy != 0.0 {
        for mut robot in query.iter_mut() {
            if robot.id == selected.0 {
                let new_x = robot.x + dx;
                let new_y = robot.y + dy;
                if new_x < 0.0 || new_y < 0.0 || (new_x as usize) >= sim_map.map.width || (new_y as usize) >= sim_map.map.height {
                    println!("Déplacement refusé : hors limites.");
                } else {
                    let cell = &sim_map.map.cells[new_y as usize][new_x as usize];
                    if *cell == Cell::Obstacle {
                        println!("Déplacement refusé : obstacle à la case ({}, {}).", new_x, new_y);
                    } else {
                        robot.x = new_x;
                        robot.y = new_y;
                        println!("Robot {} se déplace en ({}, {})", robot.id, new_x, new_y);
                    }
                }
            }
        }
    }
}