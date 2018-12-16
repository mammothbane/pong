use amethyst::core::nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 100.0,
            width: 100.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BallConfig {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            velocity: Vector2::new(75., 50.),
            radius: 2.5,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PaddleConfig {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            height: 15.,
            width: 2.5,
            speed: 75.,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct PaddlesConfig {
    pub left: PaddleConfig,
    pub right: PaddleConfig,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct PongConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddles: PaddlesConfig,
}
