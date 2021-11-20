//! Application command handler

use std::{borrow::Borrow, ops::Deref, sync::Arc, time::Duration};

use chrono::Utc;
use druid::{piet::TextStorage, AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::{
    gui::data::edit::EditState,
    recipes::{db::RecipeId, recipe::Recipe},
    SAVE_FILE,
};

use super::{
    data::{remove::RemoveState, search::SearchResults, AppState},
    CHANGE_INGREDIENT_UNIT, CHANGE_SCREEN, CREATE_RECIPE, EDIT_RECIPE, LOAD_MORE_RECIPES,
    POPULATE_RESULTS, REMOVE_EDITED_INGREDIENT, REMOVE_RECIPE, SAVE_EDITED_RECIPE, VIEW_RECIPE,
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

            data.screen = *screen;

            Handled::Yes
        } else if let Some(()) = cmd.get(POPULATE_RESULTS) {
            log::trace!("Populating search results for query");

            data.search.results = Some(SearchResults {
                recipes: data.recipes.search(|recipe| {
                    if let Some(score) = sublime_fuzzy::best_match(
                        recipe.name.borrow(),
                        data.search.query.term.as_str(),
                    )
                    .or_else(|| {
                        sublime_fuzzy::best_match(
                            recipe.body.borrow(),
                            data.search.query.term.as_str(),
                        )
                    }) {
                        score.score()
                    } else {
                        isize::MIN
                    }
                }),
                loaded_recipes: 10,
                term: Arc::from(data.search.query.term.as_str()),
            });
            Handled::Yes
        } else if let Some(recipe) = cmd.get(VIEW_RECIPE) {
            log::trace!("Viewing recipe {}...", recipe);

            data.view.viewed = Some(*recipe);
            Handled::Yes
        } else if let Some(()) = cmd.get(LOAD_MORE_RECIPES) {
            log::trace!("Loading more recipe results...");

            let ids = data.recipes.ids();
            data.home.loaded = druid::im::Vector::from(
                &ids[0..(if data.home.loaded.len() + 10 >= ids.len() {
                    ids.len()
                } else {
                    data.home.loaded.len() + 10
                })],
            );
            Handled::Yes
        } else if let Some((id, return_to)) = cmd.get(EDIT_RECIPE) {
            log::trace!("Populating edit data with recipe {}", id);
            data.edit.return_to = *return_to;
            match data.recipes.get(*id) {
                Some(recipe) => data.edit = EditState::from(recipe.deref()),
                None => log::warn!(
                    "Edit recipe command received with ID {} that does not exist",
                    id
                ),
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(CREATE_RECIPE) {
            data.edit = EditState::default();
            Handled::Yes
        } else if let Some((id, return_to)) = cmd.get(REMOVE_RECIPE) {
            if let Some(recipe) = data.recipes.get(*id) {
                data.remove = Some(RemoveState {
                    deleted: recipe,
                    return_to: *return_to,
                });
            } else {
                log::warn!("Remove recipe command received with invalid ID: {}", id);
            }
            Handled::Yes
        } else if let Some((id, unit)) = cmd.get(CHANGE_INGREDIENT_UNIT) {
            log::trace!("Changing ingredient {} unit to {}", id, unit);
            data.edit
                .ingredients
                .entry(*id)
                .and_modify(|v| v.unit = *unit);

            Handled::Yes
        } else if let Some(id) = cmd.get(REMOVE_EDITED_INGREDIENT) {
            if data.edit.ingredients.remove(id).is_none() {
                log::warn!(
                    "Remove ingredient command received with invalid ingredient id {}",
                    id
                );
            }
            Handled::Yes
        } else if let Some(()) = cmd.get(SAVE_EDITED_RECIPE) {
            let recipe = Recipe {
                name: Arc::from(data.edit.title.as_str()),
                created_on: Utc::now(),
                ingredients: data
                    .edit
                    .ingredients
                    .iter()
                    .map(|(_, edited)| edited.to_ingredient())
                    .collect(),
                servings: data.edit.servings,
                body: Arc::from(data.edit.body.as_str()),
                time: data.edit.time.map(|edited| {
                    Duration::from_secs(
                        edited.hours as u64 * 3600
                            + edited.minutes as u64 * 60
                            + edited.secs as u64,
                    )
                }),
                id: data.edit.id.unwrap_or_else(RecipeId::new),
            };
            data.recipes.insert(recipe);

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
