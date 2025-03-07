mod map;
mod robot;

use map::Map;
use robot::{Robot, RobotType};

fn main() {
    let map = Map::new(20, 10, 42);
    map.display();

    let mut robots = vec![
        Robot::new(1, RobotType::Explorer, (0, 0)),
        Robot::new(2, RobotType::CollectorEnergy, (5, 5)),
    ];

    for robot in robots.iter_mut() {
        robot.update();
    }
}
