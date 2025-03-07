#[derive(Debug)]
pub enum RobotType {
    Explorer,
    CollectorEnergy,
    CollectorMineral,
}

pub struct Robot {
    pub id: u32,
    pub robot_type: RobotType,
    pub position: (usize, usize),
}

impl Robot {
    pub fn new(id: u32, robot_type: RobotType, start_pos: (usize, usize)) -> Self {
        Robot {
            id,
            robot_type,
            position: start_pos,
        }
    }

    pub fn update(&mut self) {
        match self.robot_type {
            RobotType::Explorer => {
                println!("Robot {} (Explorer) explore à {:?}", self.id, self.position);
            },
            RobotType::CollectorEnergy => {
                println!("Robot {} (CollectorEnergy) collecte énergie à {:?}", self.id, self.position);
            },
            RobotType::CollectorMineral => {
                println!("Robot {} (CollectorMineral) collecte minerai à {:?}", self.id, self.position);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new(1, RobotType::Explorer, (0, 0));
        assert_eq!(robot.id, 1);
        assert_eq!(robot.position, (0, 0));
        // Vous pouvez ajouter d'autres assertions si vous ajoutez d'autres champs
    }
}