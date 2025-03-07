use std::sync::mpsc::Receiver;
use crate::simulation::RobotMessage;


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

    pub fn handle_message(&mut self, message: RobotMessage) {
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
        }
    }
}


pub fn run_station(rx: Receiver<RobotMessage>) {
    // Crée l'état initial de la station
    let mut station = Station::new();

    while let Ok(message) = rx.recv() {
        station.handle_message(message);
    }

    println!("Station fermée. État final : {:?}", station);
}
