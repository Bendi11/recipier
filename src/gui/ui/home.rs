//! Homescreen widgets displaying a view of all saved recipes

use druid::{Widget, WidgetExt, widget::{Button, Flex, Label, List, Scroll}};

use crate::gui::{CHANGE_SCREEN, CREATE_RECIPE, LOAD_MORE_RECIPES, data::{AppState, screen::AppScreen}, theme, widgets::{icon::{self, Icon}, separator::Separator}};

use super::recipe::recipe_brief_widget;

/// Construct a widget displaying a list of all saved recipes
pub fn home_widget() -> impl Widget<AppState> {
    let title_bar = Flex::row()
        .with_child(Label::new("Recipes")
            .with_font(theme::HEADER_FONT)
            .align_left()
        )
        .with_flex_spacer(5.)
        .with_child(Icon::svg(&icon::PLUS_ICON)
            .highlight_on_hover()
            .on_click(|ctx, _data, _env| {
                ctx.submit_command(CREATE_RECIPE);
                ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Edit));
            })
        )
        .with_spacer(10.);
        

    Flex::column()
        .with_child(title_bar.fix_height(50.).expand_width())
        .with_spacer(1.)
        .with_child(Separator::new(2.5).fix_width(130.).align_left())
        .with_default_spacer()
        .with_flex_child(Scroll::new(Flex::column()
                .with_child(List::new(|| recipe_brief_widget().padding((2., 0.))).with_spacing(10.))
                .with_default_spacer()
                .with_child(Button::new("Load More")
                    .fix_size(100., 40.)
                    .on_click(|ctx, _data, _env| ctx.submit_command(LOAD_MORE_RECIPES))
                )
            )
            .vertical(), 10.
        )
        .expand()
}