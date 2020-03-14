
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

use game;

const START_Y: f64 = 30.0;

pub struct App {
    pub gl: GlGraphics,
    pub game: game::Game
}

impl App {

    pub fn init(&self) {
        self.game.init();
    }

    pub fn render(&mut self, args: &RenderArgs) {

        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const CORNFLOWER_BLUE: [f32; 4] = [0.39, 0.58, 0.92, 1.0];

        let cell = rectangle::square(0.0, 0.0, 20.0);

        self.gl.draw(args.viewport(), |c, gl| {

            clear(CORNFLOWER_BLUE, gl);

            for i in 0..self.game.cells.len() {
                let cell = self.game.cells[i];

                let px = cell.x * game::CELL_SIZE;
                let py = cell.y * game::CELL_SIZE;

                let transform = c
                        .transform
                        .trans(px, py + START_Y);
                    
                rectangle(GREEN, cell, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

}