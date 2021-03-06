extern crate amethyst;

use::amethyst::{
    core::{
        Transform,
        SystemDesc
    },
    derive::SystemDesc,
    ecs::{
        Join,
        Read,
        ReadStorage,
        System,
        SystemData,
        World,
        WriteStorage
    },
    input::{
        InputHandler,
        StringBindings
    }
};

use crate::pong::{ Paddle, Side, ARENA_WIDTH, ARENA_HEIGHT };

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            if let Some(mv_amount) = movement {
                let scaled = 1.2 * mv_amount as f32;
                transform.prepend_translation_y(scaled);
            }
        }
    }
}
