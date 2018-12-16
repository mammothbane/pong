use amethyst::{
    prelude::*,
    renderer::{
        DisplayConfig,
        DrawFlat2D,
        Pipeline,
        RenderBundle,
        Stage,
    },
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

use self::{
    pong::Pong,
};

mod pong;
mod systems;
mod config;

fn main() -> amethyst::Result<()> {
    use amethyst::{
        core::transform::TransformBundle,
        input::InputBundle,
    };

//    amethyst::start_logger(Default::default());

    let display_config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(&display_config_path);

    let binding_path = format!("{}/resources/binding_config.ron", application_root_dir());
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawUi::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::BallMotionSystem, "ball_motion_system", &[])
        .with(systems::BallCollisionSystem, "collision_system", &["paddle_system", "ball_motion_system"])
        .with(systems::VictorySystem, "victory_system", &["paddle_system", "ball_motion_system"]);

    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    Ok(())
}
