use crossbeam::channel::{Sender, Receiver};
use std::thread;
use std::time::Duration;
use rand::Rng;

use crate::robot::{Robot, RobotType};
use crate::station::run_station;

#[derive(Debug)]
pub enum RobotMessage {
    Update(u32, String),
    ResourceCollected(u32, ResourceType, u32),
}

#[derive(Debug)]
pub enum ResourceType {
    Energy,
    Mineral,
}

#[derive(Debug)]
pub enum StationCommand {
    ProduceRobot(RobotType),
}

pub fn start_simulation(tx_robot_msg: Sender<RobotMessage>, rx_station_cmd: Receiver<StationCommand>) {
    // Ex. un petit robot qui bouge 5 fois
    let mut rng = rand::thread_rng();
    for i in 0..5 {
        tx_robot_msg.send(RobotMessage::Update(1, format!("Position: {}", i))).ok();
        if rng.gen_bool(0.5) {
            tx_robot_msg.send(RobotMessage::ResourceCollected(1, ResourceType::Energy, 1)).ok();
        }
        thread::sleep(Duration::from_millis(500));
    }

    // Lire éventuellement rx_station_cmd ici
    println!("Fin de la simulation concurrente.");
}
