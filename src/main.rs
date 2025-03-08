mod map;
mod robot;
mod simulation;
mod station;
mod gui;
mod bridge;
mod game;  
mod ui;

use bevy::prelude::*;
use gui::{setup_map, spawn_robots, move_robots};
use bridge::SimulationChannels;
use simulation::{start_simulation, RobotMessage, StationCommand};
use game::{spawn_resources, check_collection, StationState}; 
use ui::{setup_ui, update_ui};   

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(StationState::default())
        .add_systems(Startup, (
            setup_map,
            spawn_robots,
            spawn_resources,
            setup_ui,
            setup_simulation,
        ))
        .add_systems(Update, (
            move_robots,
            check_collection,
            update_ui,
            receive_simulation_msgs,
        ))
        .run();
}


fn setup_simulation(mut commands: Commands) {
    use crossbeam::channel;
    let (tx_robot_msg, rx_robot_msg) = channel::unbounded();
    let (tx_station_cmd, rx_station_cmd) = channel::unbounded();

    std::thread::spawn(move || {
        start_simulation(tx_robot_msg, rx_station_cmd);
    });

    commands.insert_resource(SimulationChannels {
        rx_robot_msgs: rx_robot_msg,
        tx_station_cmd,
    });
}

fn receive_simulation_msgs(
    mut channels: ResMut<SimulationChannels>,
    mut robot_query: Query<&mut RobotSprite>,
) {
    while let Ok(msg) = channels.rx_robot_msgs.try_recv() {
        match msg {
            RobotMessage::Update(robot_id, desc) => {
                println!("Bevy reçoit un update du robot {}: {}", robot_id, desc);
                if let Some(pos_str) = desc.strip_prefix("Position: ") {
                    if let Ok(pos_x) = pos_str.parse::<f32>() {
                        for mut sprite in robot_query.iter_mut() {
                            if sprite.id == robot_id {
                                sprite.x = pos_x;
                            }
                        }
                    }
                }
            }
            RobotMessage::ResourceCollected(robot_id, resource, amount) => {
                println!("Bevy: Robot {} a collecté {:?} (x{})", robot_id, resource, amount);
            }
        }
    }
}
