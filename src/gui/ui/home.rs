//! Homescreen widgets displaying a view of all saved recipes

use druid::{Widget, WidgetExt, widget::{Button, Flex, Label, List, Scroll}};

use crate::gui::{LOAD_MORE_RECIPES, data::AppState, theme, widgets::separator::Separator};

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
                .with_child(List::new(|| recipe_brief_widget().border(theme::COLOR_2, 2.).expand_width()))
                .with_default_spacer()
                .with_child(Button::new("Load More")
                    .on_click(|ctx, _data, _env| ctx.submit_command(LOAD_MORE_RECIPES))
                )
            )
            .vertical()
            .expand()
        )
        .expand()
}