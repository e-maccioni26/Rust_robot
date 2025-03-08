use bevy::prelude::*;
use crossbeam::channel::{Sender, Receiver};

use crate::simulation::{RobotMessage, StationCommand};

#[derive(Resource)]
pub struct SimulationChannels {
    pub rx_robot_msgs: Receiver<RobotMessage>,
    pub tx_station_cmd: Sender<StationCommand>,
}
