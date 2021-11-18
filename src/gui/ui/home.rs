//! Homescreen widgets displaying a view of all saved recipes

use druid::{Widget, WidgetExt, widget::{Flex, Label}};

use crate::gui::{data::AppState, theme, widgets::separator::Separator};

/// Construct a widget displaying a list of all saved recipes
pub fn home_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Recipes")
            .with_font(theme::HEADER_FONT)
            .align_left()
            .fix_width(50.) 
        )
        .with_spacer(1.)
        .with_child(Separator::new(1.5).fix_width(50.))
}