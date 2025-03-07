mod map;
mod robot;
mod simulation;
mod station; 

use map::Map;
use simulation::start_simulation;

fn main() {
    let map = Map::new(20, 10, 42);
    map.display();

    start_simulation();
}
