use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;

use crate::robot::{Robot, RobotType};
use crate::station::{run_station}; 

#[derive(Debug)]
pub enum RobotMessage {
    Update(u32, String), 
}

pub fn start_simulation() {

    let (tx, rx): (Sender<RobotMessage>, Receiver<RobotMessage>) = mpsc::channel();

    let station_handle = thread::spawn(move || {
        run_station(rx);
    });

    let robots = vec![
        Robot::new(1, RobotType::Explorer, (0, 0)),
        Robot::new(2, RobotType::CollectorEnergy, (5, 5)),
    ];

    let mut handles = Vec::new();
    for mut robot in robots {
        let thread_tx = tx.clone();
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                robot.update();
                thread_tx
                    .send(RobotMessage::Update(robot.id, format!("Position: {:?}", robot.position)))
                    .expect("Envoi du message impossible");
                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        let _ = handle.join();
    }

    let _ = station_handle.join();

    println!("Simulation terminée.");
}
