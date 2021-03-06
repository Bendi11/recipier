//! Application command handler

use std::{borrow::Borrow, ops::Deref, sync::Arc, time::Duration};

use chrono::Utc;
use druid::{
    commands::{CLOSE_WINDOW, OPEN_FILE, SHOW_WINDOW},
    piet::TextStorage,
    widget::{Button, Flex, Label},
    AppDelegate, Command, DelegateCtx, Env, Handled, ImageBuf, Target, WindowDesc,
};

use crate::{
    gui::data::edit::EditState,
    recipes::{db::RecipeId, recipe::Recipe},
    SAVE_FILE,
};

use super::{
    data::{remove::RemoveState, search::SearchResults, AppState},
    CHANGE_INGREDIENT_UNIT, CHANGE_SCREEN, CREATE_RECIPE, EDIT_RECIPE, LOAD_MORE_RECIPES,
    POPULATE_RESULTS, REMOVE_EDITED_INGREDIENT, REMOVE_RECIPE, SAVE_EDITED_RECIPE,
    SHOW_UPDATE_DIALOG, VIEW_RECIPE,
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
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some((channel, version)) = cmd.get(SHOW_UPDATE_DIALOG) {
            use druid::WidgetExt;

            let channel_cancel = channel.clone();
            let channel_update = channel.clone();
            let version = version.clone();

            let update_window = WindowDesc::new(move || Flex::column()
                .with_default_spacer()
                .with_child(Label::dynamic(move |_state: &AppState, _| {
                    format!(
                        "Recipier version {} is available to update, would you like to update and restart?",
                        version
                    )
                }))
                .with_default_spacer()
                .with_child(
                    Flex::row()
                        .with_default_spacer()
                        .with_child(
                            Button::new("Don't Update")
                                .on_click(move |ctx, _data: &mut AppState, _| {
                                    if let Err(e) = channel_cancel.send(false) {
                                        log::error!("Failed to send cancel signal to updater thread: {}", e);
                                        
                                    }
                                    let id = ctx.window_id();
                                    ctx.submit_command(CLOSE_WINDOW.to(id));
                                }),
                        )
                        .with_flex_spacer(1.0)
                        .with_child(
                            Button::new("Restart and Update")
                                .on_click(move |ctx, _data: &mut AppState, _| {
                                        if let Err(e) = channel_update.send(true) {
                                        log::error!("Failed to send update signal to updater thread: {}", e);
                                    }
                                    let id = ctx.window_id();
                                    ctx.submit_command(CLOSE_WINDOW.to(id));
                                }),
                        )
                        .padding((10., 0.)),
                )
                .with_default_spacer()
            )
                .title("Update")
                .show_titlebar(false)
                .resizable(false)
                .window_size((600., 480.));
            let id = update_window.id;

            ctx.new_window(update_window);
            ctx.submit_command(SHOW_WINDOW.to(Target::Window(id)));

            Handled::Yes
        } else if let Some(screen) = cmd.get(CHANGE_SCREEN) {
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
            data.home.loaded =
                druid::im::Vector::from(&ids[0..ids.len().min(data.home.loaded.len() + 10)]);
            Handled::Yes
        } else if let Some((id, return_to)) = cmd.get(EDIT_RECIPE) {
            log::trace!("Populating edit data with recipe {}", id);
            data.edit.return_to = *return_to;
            match data.recipes.get(*id) {
                Some(recipe) => data.edit = EditState::from_recipe(&data.recipes, recipe.deref()),
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
            let recipe_id = data.edit.id.unwrap_or_else(RecipeId::new);

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
                id: recipe_id,
            };
            data.recipes.insert(recipe);

            if let Some(ref img) = data.edit.image {
                data.recipes.set_image(recipe_id, img.clone())
            }

            Handled::Yes
        } else if let Some(info) = cmd.get(OPEN_FILE) {
            use druid::image;
            match image::open(info.path()) {
                Ok(img) => {
                    let img = img.to_rgba8();
                    let width = img.width() as usize;
                    let height = img.height() as usize;
                    let buf = ImageBuf::from_raw(
                        img.into_raw(),
                        druid::piet::ImageFormat::RgbaSeparate,
                        width,
                        height,
                    );

                    data.edit.image = Some(buf);
                }
                Err(e) => {
                    log::trace!(
                        "File {} can not be used as an image: {}",
                        info.path().display(),
                        e
                    );
                }
            }

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
        if let druid::Event::WindowSize(size) = event {
            data.config.window_size = (size.width, size.height)
        }
        Some(event)
    }
}
