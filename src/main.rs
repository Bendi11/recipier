// Disable console on windows
#![windows_subsystem = "windows"]

mod updater;
mod gui;



fn main() {
    //Add panic handler for better error messages
    std::panic::set_hook(Box::new(|info| {

    }));
}