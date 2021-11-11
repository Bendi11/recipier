//! Widgets for displaying recipes

use std::sync::Arc;

use druid::{LensExt, Widget, WidgetExt, lens, widget::{Flex, Label}};

use crate::{gui::theme, recipes::recipe::Recipe};

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget() -> impl Widget<Arc<Recipe>> {
    let layout = Flex::column()
        .with_child(Label::raw()
            .with_font(theme::LABEL_FONT)
            .lens(Recipe::name)
        )
        .with_spacer(0.1)
        .with_child(Label::new(|data: &Recipe, _env: &'_ _| {
            format!("Created {}", data.created_on.format("%e %B %Y %I:%M"))
        }))
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity));
        

    layout
}