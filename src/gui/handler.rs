//! Application command handler

use std::{borrow::Borrow, sync::Arc};

use druid::{im::Vector, AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::SAVE_FILE;

use super::{
    data::{screen::AppScreen, search::SearchResults, AppState},
    CHANGE_SCREEN, POPULATE_RESULTS,
};

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

    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(screen) = cmd.get(CHANGE_SCREEN) {
            if *screen == data.screen {
                return Handled::Yes;
            }

            match screen {
                AppScreen::Home => data.screen = *screen,
                &AppScreen::SearchResults => data.screen = *screen,
            }

            Handled::Yes
        } else if let Some(()) = cmd.get(POPULATE_RESULTS) {
            data.search.results = Some(SearchResults {
                recipes: data.recipes.search(|recipe| {
                    if let Some(score) = sublime_fuzzy::best_match(recipe.name.borrow(), data.search.query.term.as_str())
                        .or_else(|| sublime_fuzzy::best_match(recipe.body.borrow(), data.search.query.term.as_str())) {
                        score.score()
                    } else {
                        isize::MIN
                    }
                }),
                loaded_recipes: 10,
                term: Arc::from(data.search.query.term.as_str()),
            });
            Handled::Yes
        } else {
            Handled::No
        }
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
