// Disable console on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use druid::{AppLauncher, WindowDesc, WindowState};
use gui::{data::AppState, root_widget};
use log::LevelFilter;
use simplelog::ConfigBuilder;
pub mod gui;
pub mod recipes;

/// The file name to save and load application data from
pub const SAVE_FILE: &str = "./save.json";

fn main() {
    //Add panic handler for better error messages
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "A fatal error occurred in {}: {}",
            match info.location() {
                Some(loc) => loc.to_string(),
                None => "[unknown location]".to_owned(),
            },
            match (
                info.payload().downcast_ref::<&str>(),
                info.payload().downcast_ref::<String>()
            ) {
                (Some(err), _) => (*err).to_owned(),
                (_, Some(err)) => err.clone(),
                (None, None) => "unknown error".to_owned(),
            }
        );
    }));

    match std::fs::File::create("./recipier.log") {
        Ok(log) => {
            if let Err(e) =
                simplelog::WriteLogger::init(LevelFilter::max(), ConfigBuilder::new().build(), log)
            {
                eprintln!("failed to initialize logger: {}", e);
            }
        }
        Err(e) => {
            if let Err(logerr) = simplelog::TermLogger::init(
                LevelFilter::max(),
                ConfigBuilder::new().build(),
                simplelog::TerminalMode::Stderr,
                simplelog::ColorChoice::Auto,
            ) {
                eprintln!(
                    "failed to open log file: {}\nand to initialize terminal logger: {}",
                    e, logerr
                );
            }
            log::info!(
                "failed to open log file at ./recipier.log: {}, logging to terminal instead",
                e
            );
        }
    }

    let state = AppState::init(SAVE_FILE);

    let window = WindowDesc::new(root_widget)
        .resizable(true)
        .title("Recipier")
        .set_window_state(WindowState::RESTORED)
        .window_size(state.config.window_size);

    //let icon = image::load_from_memory_with_format(ICON, image::ImageFormat::Png).unwrap();
    if let Err(e) = AppLauncher::with_window(window)
        .configure_env(|env, _state| gui::theme::set(env))
        .delegate(gui::handler::RecipierDelegate)
        .launch(state)
    {
        panic!("Failed to launch app: {}", e);
    }
}
