//! Widgets for displaying recipes

use std::sync::Arc;

use druid::{LensExt, Widget, WidgetExt, widget::Label};

use crate::{gui::{data::AppState, theme, widgets::maybe::Maybe}, recipes::{db::RecipeId, recipe::Recipe}};

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget(id: RecipeId) -> impl Widget<AppState> {
    let title = Maybe::<Arc<Recipe>>::or_empty(|| Label::raw()
            .with_font(theme::LABEL_FONT)
            .lens(druid::lens::Identity.in_arc().then(Recipe::name))
        )
        .lens(AppState::recipies.map(
            |db| db.get(id), 
            |db, recipe| if let Some(recipe) = recipe {
            db.update(id, recipe)
        })
    );

    title
}