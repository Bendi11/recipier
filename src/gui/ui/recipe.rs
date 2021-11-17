//! Widgets for displaying recipes

use std::sync::Arc;

use druid::{LensExt, TextAlignment, Widget, WidgetExt, lens, widget::{Flex, Label, LineBreaking, List, Scroll}};

use crate::{gui::{CHANGE_SCREEN, VIEW_RECIPE, data::{AppState, screen::AppScreen}, theme, widgets::{maybe::Maybe, separator::Separator}}, recipes::recipe::{Ingredient, Recipe}};

/// The string to use when formatting chrono datetimes
pub const DATETIME_FORMAT: &str = "%e %B %Y %I:%M";

/// Return a widget that displays one recipe in a maximized view
pub fn view_screen() -> impl Widget<AppState> {
    Maybe::or_empty(recipe_widget)
        .lens(lens::Identity.map(
        |state: &AppState| state.recipes.get(state.view.viewed?), 
        |state, recipe| if let Some(recipe) = recipe {
            state.recipes.update(recipe);
        }))
}

/// Show a widget that displays all information about the recipe
pub fn recipe_widget() -> impl Widget<Arc<Recipe>> {
    Flex::column()
        .with_child(Label::raw()
            .with_font(theme::HEADER_FONT)
            .with_line_break_mode(LineBreaking::WordWrap)
            .align_left()
            .lens(Recipe::name)
        )
        .with_default_spacer()
        .with_child(Label::new(|recipe: &Recipe, _env: &'_ _| {
                format!("Created {}", recipe.created_on.format(DATETIME_FORMAT))
            })
            .align_left()
        )
        .with_default_spacer()
        .with_child(Separator::new(2.))
        .with_default_spacer()
        .with_child(Label::new("Ingredients").with_font(theme::LABEL_FONT).align_left())
        .with_default_spacer()
        .with_child(Scroll::new(List::new(|| {
            Flex::column()
                .with_child(Flex::row()
                    .with_child(Label::raw().with_font(theme::SYSTEM_FONT).lens(Ingredient::name).align_left())
                    .with_default_spacer()
                    .with_child(Label::new(|ingredient: &Ingredient, _env: &'_ _| format!("{}", ingredient.amount)))
                    .expand_width()
                    .border(theme::COLOR_2, 2.)
                    .rounded(5.0)
                )
                .with_default_spacer()
        })).vertical())
        .with_default_spacer()
        .with_child(Label::raw()
            .with_font(theme::SYSTEM_FONT)
            .with_line_break_mode(LineBreaking::WordWrap)
            .with_text_alignment(TextAlignment::Start)
            .expand()
            .padding((5., 5.))
            .background(theme::COLOR_2)
            .lens(Recipe::body)
        )
        .expand()
        
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
}

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget() -> impl Widget<Arc<Recipe>> {
    Flex::column()
        .with_child(
            Label::raw()
                .with_font(theme::LABEL_FONT)
                .lens(Recipe::name)
                .align_left()
                .on_click(|ctx, recipe, _env| {
                    ctx.submit_command(VIEW_RECIPE.with(recipe.id));
                    ctx.submit_command(CHANGE_SCREEN.with(AppScreen::View));
                }),
        )
        .with_spacer(0.1)
        .with_child(Label::new(|data: &Recipe, _env: &'_ _| {
            format!("Created {}", data.created_on.format(DATETIME_FORMAT))
        }))
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
}
