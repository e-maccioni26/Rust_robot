use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::simulation::{RobotMessage, StationCommand, ResourceType};
use crate::robot::RobotType;

#[derive(Debug)]
pub struct Station {
    pub energy_collected: u32,
    pub minerals_collected: u32,
    pub known_positions: Vec<(u32, (usize, usize))>,
}

impl Station {
    pub fn new() -> Self {
        Station {
            energy_collected: 0,
            minerals_collected: 0,
            known_positions: Vec::new(),
        }
    }

    pub fn handle_message(&mut self, message: RobotMessage, cmd_tx: &Sender<StationCommand>) {
        match message {
            RobotMessage::Update(id, desc) => {
                if let Some(pos_str) = desc.strip_prefix("Position: ") {
                    let parsed = pos_str.trim_matches(|c| c == '(' || c == ')' || c == ' ');
                    if let Some((x, y)) = parsed.split_once(",") {
                        let x = x.trim().parse::<usize>().unwrap_or(0);
                        let y = y.trim().parse::<usize>().unwrap_or(0);
                        self.known_positions.push((id, (x, y)));
                    }
                }
                println!("Station stocke info du robot {}: {}", id, desc);
            }
            RobotMessage::ResourceCollected(id, resource, amount) => {
                match resource {
                    ResourceType::Energy => {
                        self.energy_collected += amount;
                        println!(
                            "Station: Robot {} a collecté {} unité(s) d'énergie. Total énergie: {}",
                            id, amount, self.energy_collected
                        );
                    }
                    ResourceType::Mineral => {
                        self.minerals_collected += amount;
                        println!(
                            "Station: Robot {} a collecté {} unité(s) de minerai. Total minerai: {}",
                            id, amount, self.minerals_collected
                        );
                    }
                }

                if self.energy_collected >= 2 {
                    println!("Station: Seuil atteint pour la production d'un nouveau robot.");
                    cmd_tx
                        .send(StationCommand::ProduceRobot(RobotType::Explorer))
                        .expect("Impossible d'envoyer la commande de production.");
                    self.energy_collected -= 2;
                }
            }
        }
    }
}


pub fn run_station(rx: Receiver<RobotMessage>, cmd_tx: Sender<StationCommand>) {
    let mut station = Station::new();
    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(message) => {
                station.handle_message(message, &cmd_tx);
            }
            Err(_) => {
                println!("Station: plus de messages, fermeture.");
                break;
            }
        }
    }
    println!("Station fermée. État final : {:?}", station);
}
