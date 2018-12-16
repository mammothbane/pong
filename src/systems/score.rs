use amethyst::{
    core::{
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::*,
    ui::UiText,
};

use crate::{
    config::ArenaConfig,
    pong::{
        Ball,
        LastScoreTime,
        Scoreboard,
        ScoreText,
    },
};

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, Scoreboard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, ArenaConfig>,
        Read<'s, Time>,
        Write<'s, LastScoreTime>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scoreboard,
        text,
        arena_config,
        time,
        mut last_score,
    ): Self::SystemData) {
        for (mut ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let mut scored = false;

            if ball_x < ball.radius {
                scoreboard.score_right = (scoreboard.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(text.p2_score) {
                    text.text = scoreboard.score_right.to_string();
                }

                scored = true;
            }

            if ball_x >= arena_config.width - ball.radius {
                scoreboard.score_left = (scoreboard.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(text.p1_score) {
                    text.text = scoreboard.score_left.to_string();
                }

                scored = true;
            }

            if scored {
                last_score.0 = time.absolute_time();

                ball.velocity[0] = -ball.velocity[0];
                transform.set_x(arena_config.width / 2.0);
            }
        }
    }
}
