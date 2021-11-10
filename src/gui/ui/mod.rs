pub mod search;

use druid::{LensExt, Widget, WidgetExt, widget::{Button, Flex, Label}};

use crate::gui::{
    theme,
    widgets::{
        icon::{self, Icon},
        separator::Separator,
    },
};

use super::{GOLDEN_RATIO, data::{AppState, search::{Query, SearchState}}};

pub fn root_widget() -> impl Widget<AppState> {
    
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_flex_child(
            Icon::svg(&icon::BOWL_ICON)
                .with_scale(10.)
                .with_color(theme::COLOR_4), 0.1)
        .with_child(Separator::new(5.))
        .with_default_spacer()
        .with_child(search::search_bar().lens(AppState::search.then(SearchState::query).then(Query::term)))
        .with_default_spacer()
        .with_child(Button::new("Test").expand_width())
        .padding((5., 0., 0., 0.));

    let screen = Flex::row()
        .with_flex_child(sidebar, 1.)
        .with_default_spacer()
        .with_child(Separator::new(5.).vertical(true).with_color(theme::COLOR_2))
        .with_default_spacer()
        .with_flex_child(Label::new("Body"), GOLDEN_RATIO);
    

    screen
}
