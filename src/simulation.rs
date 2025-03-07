use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use crate::robot::{Robot, RobotType};

pub enum RobotMessage {
    Update(u32, String), 
}

pub fn start_simulation() {
    let (tx, rx) = mpsc::channel();

    // Créez quelques robots
    let mut robots = vec![
        Robot::new(1, RobotType::Explorer, (0, 0)),
        Robot::new(2, RobotType::CollectorEnergy, (5, 5)),
    ];

    for mut robot in robots {
        let thread_tx: Sender<RobotMessage> = tx.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                robot.update();
                thread_tx
                    .send(RobotMessage::Update(robot.id, format!("Position: {:?}", robot.position)))
                    .expect("Échec de l'envoi du message");
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    drop(tx);
    for received in rx {
        match received {
            RobotMessage::Update(id, desc) => {
                println!("Station reçoit update du robot {}: {}", id, desc);
            }
        }
    }
}
