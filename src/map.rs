#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    ScientificInterest,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut cells = vec![vec![Cell::Empty; width]; height];

        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(seed as u64);

        for y in 0..height {
            for x in 0..width {
                let roll: f64 = rng.gen();
                
                cells[y][x] = if roll < 0.2 {
                    Cell::Obstacle
                } else if roll < 0.25 {
                    Cell::Energy
                } else if roll < 0.3 {
                    Cell::Mineral
                } else if roll < 0.32 {
                    Cell::ScientificInterest
                } else {
                    Cell::Empty
                };
            }
        }

        Map { width, height, cells }
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::Obstacle => "#",
                    Cell::Energy => "E",
                    Cell::Mineral => "M",
                    Cell::ScientificInterest => "S",
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_reproducibility() {
        // Deux cartes générées avec la même seed doivent être identiques
        let map1 = Map::new(10, 10, 123);
        let map2 = Map::new(10, 10, 123);
        assert_eq!(map1.cells, map2.cells, "Les cartes doivent être identiques pour la même seed");
    }
}
