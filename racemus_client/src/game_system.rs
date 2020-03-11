
use amethyst::{
    prelude::*,
    core::SystemDesc,
    derive::SystemDesc,
    input::{
        InputHandler,
        VirtualKeyCode,
        StringBindings
    },

}

#[derivce(SystemDesc)]
struct GameSystem;

impl<'s> System<'s> for GameSystem {

    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {

        if let Some((x,y)) = input.
        }

    }
}
