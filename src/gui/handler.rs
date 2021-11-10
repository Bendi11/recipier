//! Application command handler

use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::SAVE_FILE;

use super::{CHANGE_SCREEN, data::AppState};

/// Structure that handles top-level events and commands in the application
pub struct RecipierDelegate;

impl AppDelegate<AppState> for RecipierDelegate {
    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        match std::fs::File::create(SAVE_FILE) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer(file, &data) {
                    log::error!("Failed to serialize app state: {}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to open save file: {}", e);
            }
        }
    }


    fn command(&mut self, ctx: &mut DelegateCtx, target: Target, cmd: &Command, data: &mut AppState, env: &Env) -> Handled {
        if let Some(payload) = cmd.get(CHANGE_SCREEN) {

        }
        
        Handled::No
    }

    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        data: &mut AppState,
        _env: &Env,
    ) -> Option<druid::Event> {
        match event {
            druid::Event::WindowSize(size) => data.config.window_size = (size.width, size.height),
            _ => (),
        }
        Some(event)
    }
}
