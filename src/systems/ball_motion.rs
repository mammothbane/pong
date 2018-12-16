use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
};

use crate::pong::Ball;

pub struct BallMotionSystem;

impl<'s> System<'s> for BallMotionSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        let time_delta = time.delta_seconds();

        (&balls, &mut locals)
            .join()
            .for_each(|(ball, local)| {
                local.translate_x(ball.velocity[0] * time_delta);
                local.translate_y(ball.velocity[1] * time_delta);
            });
    }
}