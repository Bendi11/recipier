//! Widgets for displaying recipes

use std::sync::Arc;

use druid::{LensExt, Widget, WidgetExt, lens, widget::{Flex, Label, LineBreaking, List, Scroll}};

use crate::{gui::{theme, widgets::separator::Separator}, recipes::recipe::{Ingredient, Recipe}};

/// The string to use when formatting chrono datetimes
pub const DATETIME_FORMAT: &str = "%e %B %Y %I:%M";

/// Show a widget that displays all information about the recipe
pub fn recipe_widget() -> impl Widget<Arc<Recipe>> {
    Flex::column()
        .with_child(Label::raw()
            .with_font(theme::HEADER_FONT)
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(Recipe::name)
        )
        .with_default_spacer()
        .with_child(Label::new(|recipe: &Recipe, _env: &'_ _| {
                format!("Created {}", recipe.created_on.format(DATETIME_FORMAT))
            })
        )
        .with_default_spacer()
        .with_child(Separator::new(2.))
        .with_child(Scroll::new(List::new(|| {
            Flex::row()
                .with_child(Label::raw().with_font(theme::SYSTEM_FONT).lens(Ingredient::name))
                .with_default_spacer()
                .with_child(Label::new(|ingredient: &Ingredient, _env: &'_ _| format!("{}", ingredient.amount)))
                .expand_width()
                .border(theme::COLOR_2, 2.)
                .rounded(5.0)
        })))
        
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
}

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget() -> impl Widget<Arc<Recipe>> {
    Flex::column()
        .with_child(
            Label::raw()
                .with_font(theme::LABEL_FONT)
                .lens(Recipe::name)
                .align_left(),
        )
        .with_spacer(0.1)
        .with_child(Label::new(|data: &Recipe, _env: &'_ _| {
            format!("Created {}", data.created_on.format(DATETIME_FORMAT))
        }))
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
}
