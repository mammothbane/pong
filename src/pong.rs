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

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub const HEIGHT: f32 = 16.0;
    pub const WIDTH: f32 = 4.0;

    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: Paddle::WIDTH,
            height: Paddle::HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub const RADIUS: f32 = 2.0;
    pub const VELOCITY_X: f32 = 75.0;
    pub const VELOCITY_Y: f32 = 50.0;
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

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet = load_sprite_sheet(world);

        init_paddles(world, sprite_sheet.clone());
        init_ball(world, sprite_sheet);
        init_scoreboard(world);
        init_camera(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(
            Projection::orthographic(
                0.0,
                ARENA_WIDTH,
                0.0,
                ARENA_HEIGHT,
            )
        ))
        .with(transform)
        .build();
}

fn init_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_tsfm = Transform::default();
    let mut right_tsfm = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_tsfm.set_xyz(Paddle::WIDTH * 0.5, y, 0.0);
    right_tsfm.set_xyz(ARENA_WIDTH - Paddle::WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(left_tsfm)
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render.clone())
        .with(right_tsfm)
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
    let mut local = Transform::default();
    local.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: Ball::RADIUS,
            velocity: [Ball::VELOCITY_X, Ball::VELOCITY_Y],
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