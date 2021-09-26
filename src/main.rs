// Disable console on windows
#![windows_subsystem = "windows"]

use image::GenericImageView;
pub mod gui;
pub mod recipe;
pub mod measure;

const ICON: &[u8] = include_bytes!("../assets/icon.png");

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
    let app = gui::RecipeApp::load_or_default();
    let mut opts = eframe::NativeOptions::default();

    let icon = image::load_from_memory_with_format(ICON, image::ImageFormat::Png).unwrap();
    
    opts.icon_data = Some(eframe::epi::IconData {
        rgba: icon.as_rgba8().unwrap().to_vec(),
        width: icon.width(),
        height: icon.height()
    });
    eframe::run_native(Box::new(app), opts);
    
}
