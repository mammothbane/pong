use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
};

use crate::pong::{
    ARENA_HEIGHT,
    Ball,
    Paddle,
    Side,
};

pub struct BallCollisionSystem;

impl <'s> System<'s> for BallCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (mut ball, transform) in (&mut balls, &transforms).join() {
            let pos = transform.translation();

            let above_top = pos.y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0;
            let below_bottom = pos.y <= ball.radius && ball.velocity[1] < 0.0;

            if above_top || below_bottom {
                ball.velocity[1] = -ball.velocity[1];
            }

            for (paddle, p_transform) in (&paddles, &transforms).join() {
                let paddle_x = p_transform.translation().x - paddle.width * 0.5;
                let paddle_y = p_transform.translation().y - paddle.height * 0.5;

                if !point_in_rect(pos.x, pos.y,
                                 paddle_x - ball.radius,
                                 paddle_y - ball.radius,
                                 paddle_x + paddle.width + ball.radius,
                                 paddle_y + paddle.height + ball.radius) { continue }

                let bounce = match paddle.side {
                    Side::Left => ball.velocity[0] < 0.0,
                    Side::Right => ball.velocity[0] > 0.0,
                };

                if bounce {
                    ball.velocity[0] = -ball.velocity[0];
                }
            }
        }
    }
}

#[inline]
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}