use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    ui::UiText,
};

use crate::pong::{
    ARENA_WIDTH,
    Ball,
    Scoreboard,
    ScoreText,
};

pub struct VictorySystem;

impl<'s> System<'s> for VictorySystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, Scoreboard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (mut balls, mut locals, mut ui_text, mut scoreboard, text): Self::SystemData) {
        for (mut ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let mut scored = false;

            if ball_x < ball.radius {
                scoreboard.score_right = (scoreboard.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(text.p1_score) {
                    text.text = scoreboard.score_left.to_string();
                }

                scored = true;
            }

            if ball_x >= ARENA_WIDTH - ball.radius {
                scoreboard.score_left = (scoreboard.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(text.p2_score) {
                    text.text = scoreboard.score_right.to_string();
                }

                scored = true;
            }

            if scored {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_x(ARENA_WIDTH / 2.0);

                println!("Score: | {:^3} | {:^3} |", scoreboard.score_left, scoreboard.score_right);
            }
        }
    }
}
