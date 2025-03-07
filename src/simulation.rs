use std::sync::mpsc::{self, Sender, Receiver};
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

pub fn start_simulation() {

    let (cmd_tx, cmd_rx): (Sender<StationCommand>, Receiver<StationCommand>) = mpsc::channel();

    let (tx, rx): (Sender<RobotMessage>, Receiver<RobotMessage>) = mpsc::channel();

    let station_handle = thread::spawn({
        let cmd_tx_clone = cmd_tx.clone();
        move || {
            run_station(rx, cmd_tx_clone);
        }
    });

    let mut robot_handles = Vec::new();
    let initial_robots = vec![
        Robot::new(1, RobotType::Explorer, (0, 0)),
        Robot::new(2, RobotType::CollectorEnergy, (5, 5)),
    ];

    for mut robot in initial_robots {
        let thread_tx = tx.clone();
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..5 {
                robot.update();
                thread_tx
                    .send(RobotMessage::Update(robot.id, format!("Position: {:?}", robot.position)))
                    .expect("Envoi du message update impossible");
                if let RobotType::CollectorEnergy = robot.robot_type {
                    if rng.gen_bool(0.5) {
                        thread_tx
                            .send(RobotMessage::ResourceCollected(robot.id, ResourceType::Energy, 1))
                            .expect("Envoi du message resource impossible");
                        println!("Robot {} collecte 1 unité d'énergie", robot.id);
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
        robot_handles.push(handle);
    }

    for handle in robot_handles {
        let _ = handle.join();
    }

    let mut new_robot_handles = Vec::new();
    let mut next_robot_id = 100; 
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            StationCommand::ProduceRobot(robot_type) => {
                println!("Simulation: Production d'un nouveau robot: {:?}", robot_type);
                let tx_clone = tx.clone();
                let robot_id = next_robot_id;
                next_robot_id += 1;
                let handle = thread::spawn(move || {
                    let mut rng = rand::thread_rng();
                    let mut new_robot = Robot::new(robot_id, robot_type, (0, 0));
                    for _ in 0..5 {
                        new_robot.update();
                        tx_clone
                            .send(RobotMessage::Update(new_robot.id, format!("Position: {:?}", new_robot.position)))
                            .expect("Envoi du message update impossible");
                        thread::sleep(Duration::from_millis(500));
                    }
                });
                new_robot_handles.push(handle);
            }
        }
    }

    for handle in new_robot_handles {
        let _ = handle.join();
    }

    drop(tx);

    let _ = station_handle.join();

    println!("Simulation terminée.");
}
