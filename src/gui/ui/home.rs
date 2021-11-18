//! Homescreen widgets displaying a view of all saved recipes

use druid::{Widget, WidgetExt, widget::{Flex, Label, List, Scroll}};

use crate::gui::{data::AppState, theme, widgets::separator::Separator};

use super::recipe::recipe_brief_widget;

/// Construct a widget displaying a list of all saved recipes
pub fn home_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Recipes")
            .with_font(theme::HEADER_FONT)
            .align_left()
        )
        .with_spacer(1.)
        .with_child(Separator::new(2.5).fix_width(130.).align_left())
        .with_default_spacer()
        .with_child(Scroll::new(Flex::column()
                .with_child(List::new(recipe_brief_widget)
                )
            )
            .vertical()
            .expand()
        )
        .expand()
}