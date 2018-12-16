use amethyst::core::nalgebra::{
    Vector2,
    Vector4,
};
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
    pub color: Vector4<f32>,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            velocity: Vector2::new(75., 50.),
            radius: 2.5,
            color: Vector4::new(1., 0., 0., 1.),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PaddleConfig {
    pub height: f32,
    pub width: f32,
    pub velocity: f32,
    pub color: Vector4<f32>,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            height: 15.,
            width: 2.5,
            velocity: 75.,
            color: Vector4::new(0., 0., 1., 1.),
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
