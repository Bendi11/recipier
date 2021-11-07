// Disable console on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use druid::{AppLauncher, Color, WindowDesc, WindowState, theme};
use gui::{root_widget, state::State};
use log::LevelFilter;
use simplelog::ConfigBuilder;
pub mod recipes;
pub mod gui;

//const ICON: &[u8] = include_bytes!("../assets/icon.png");

fn main() {
    //Add panic handler for better error messages
    std::panic::set_hook(Box::new(|info| {
        eprintln!("A fatal error occurred in {}: {}", match info.location() {
            Some(loc) => loc.to_string(),
            None => "[unknown location]".to_owned()
        }, match (info.payload().downcast_ref::<&str>(), info.payload().downcast_ref::<String>()) {
            (Some(err), _) => (*err).to_owned(),
            (_, Some(err)) => err.clone(),
            (None, None) => "unknown error".to_owned()
        });
    }));
    
    match std::fs::File::create("./recipier.log") {
        Ok(log) => if let Err(e) = simplelog::WriteLogger::init(LevelFilter::max(), ConfigBuilder::new().build(), log) {
            eprintln!("Failed to initialize logger: {}", e);
        },
        Err(e) => {
            eprintln!("Failed to open log file at ./recipier.log: {}", e);
        }
    }
    
    
    let window = WindowDesc::new(root_widget)
        .resizable(true)
        .title("Recipier")
        .set_window_state(WindowState::RESTORED);
    let state = State::init("./save.json");

    //let icon = image::load_from_memory_with_format(ICON, image::ImageFormat::Png).unwrap();
    if let Err(e) = AppLauncher::with_window(window).configure_env(|env, _state| {
        env.set(theme::BACKGROUND_DARK, Color::AQUA);
    }).launch(state) {
        panic!("Failed to launch app: {}", e);
    }
}
