
pub const CELL_SIZE: f64 = 20.0;

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub alive: bool
}

impl Cell {

    pub fn new() -> Self {
        Cell {
            x: 0,
            y: 0,
            alive: true
        }
    }

}

pub struct Game {
    pub cells: Vec<Cell>
}

impl Game {

    pub fn new() -> Self {
        Game {
            cells: Vec::new()
        }
    }

    pub fn init(&self) {

        for x in 0..100 {
            let cell = Cell {
                x: x,
                y: x,
                alive: true
            };
        }

    }
}


