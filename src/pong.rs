use std::time::Duration;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera,
        Projection,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
    },
};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Scoreboard {
    pub score_left: u32,
    pub score_right: u32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct LastScoreTime(pub Duration);

impl Default for LastScoreTime {
    fn default() -> Self {
        LastScoreTime(Duration::new(0, 0))
    }
}


pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet = load_sprite_sheet(world);

        load_config(world);

        world.add_resource(LastScoreTime::default());

        init_paddles(world, sprite_sheet.clone());
        init_ball(world, sprite_sheet);
        init_scoreboard(world);
        init_camera(world);
    }
}

fn load_config(world: &mut World) {
    use amethyst::utils::application_root_dir;
    use crate::config::PongConfig;

    let config_path = format!("{}/resources/config.ron", application_root_dir());
    let config = PongConfig::load_no_fallback(&config_path).unwrap();

    world.add_resource(config.arena);
    world.add_resource(config.ball);
    world.add_resource(config.paddles);
}

fn init_camera(world: &mut World) {
    use crate::config::ArenaConfig;

    let (width, height) = {
        let arena_config = world.read_resource::<ArenaConfig>();
        (arena_config.width, arena_config.height)
    };

    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(
            Projection::orthographic(
                0.0,
                width,
                0.0,
                height,
            )
        ))
        .with(transform)
        .build();
}

fn init_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    use crate::config::{ArenaConfig, PaddlesConfig};

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let arena_config = *world.read_resource::<ArenaConfig>();
    let paddle_config = *world.read_resource::<PaddlesConfig>();

    let y = arena_config.height / 2.0;
    left_transform.set_xyz(paddle_config.left.width * 0.5, y, 0.0);
    right_transform.set_xyz(arena_config.width - paddle_config.right.width * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Paddle {
            side: Side::Left,
            width: paddle_config.left.width,
            height: paddle_config.left.height,
            speed: paddle_config.left.speed,
        })
        .with(sprite_render.clone())
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(Paddle {
            side: Side::Right,
            width: paddle_config.right.width,
            height: paddle_config.right.height,
            speed: paddle_config.right.speed,
        })
        .with(sprite_render.clone())
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    use amethyst::renderer::PngFormat;

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn init_ball(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    use crate::config::{ArenaConfig, BallConfig};

    let arena_config = *world.read_resource::<ArenaConfig>();
    let ball_config = *world.read_resource::<BallConfig>();

    let mut local = Transform::default();
    local.set_xyz(arena_config.width / 2.0, arena_config.height / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: ball_config.radius,
            velocity: [ball_config.velocity.x, ball_config.velocity.y],
        })
        .with(local)
        .build();
}

fn init_scoreboard(world: &mut World) {
    use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

    let font = world.read_resource::<Loader>()
        .load(
            "font/square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0,
    );

    let p2_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        50., -50., 1., 200., 50., 0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { p1_score, p2_score });
}