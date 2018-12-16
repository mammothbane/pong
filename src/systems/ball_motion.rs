use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::*,
};

use crate::{
    config::BallConfig,
    pong::{
        Ball,
        LastScoreTime,
    },
};

pub struct BallMotionSystem;

impl<'s> System<'s> for BallMotionSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, LastScoreTime>,
        Read<'s, BallConfig>,
    );

    fn run(&mut self, (balls, mut locals, time, last_score, ball_config): Self::SystemData) {
        let diff = time.absolute_time() - last_score.0;
        if (diff.as_float_secs() as f32) < ball_config.score_timeout_sec {
            return
        }

        let time_delta = time.delta_seconds();

        (&balls, &mut locals)
            .join()
            .for_each(|(ball, local)| {
                local.translate_x(ball.velocity[0] * time_delta);
                local.translate_y(ball.velocity[1] * time_delta);
            });
    }
}