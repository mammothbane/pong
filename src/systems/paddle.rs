use amethyst::{
    core::Transform,
    ecs::{
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage,
    },
    input::InputHandler,
};

use crate::{
    config::ArenaConfig,
    pong::{
        Paddle,
        Side,
    }
};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, paddles, input, arena_config): Self::SystemData) {
        (&paddles, &mut transforms)
            .join()
            .filter_map(|(paddle, transform)| {
                let amount = match paddle.side {
                    Side::Left => input.axis_value("left_paddle"),
                    Side::Right => input.axis_value("right_paddle"),
                };

                amount.map(|amt| (paddle, transform, amt))
            })
            .for_each(|(paddle, transform, amt)| {
                let y = transform.translation().y;

                transform.set_y(num::clamp(y + 1.2 * amt as f32,
                                           paddle.height * 0.5,
                                           arena_config.height - paddle.height * 0.5,
                                           )
                );
            });
    }
}
