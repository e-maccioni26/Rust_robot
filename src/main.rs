mod map;

use map::Map;

fn main() {
    let map = Map::new(20, 10, 42);
    map.display();
}
