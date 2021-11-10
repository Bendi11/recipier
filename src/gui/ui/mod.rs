use druid::{Widget, WidgetExt, widget::{Flex, Label}};

use crate::gui::{
    theme,
    widgets::{
        icon::{self, Icon},
        separator::Separator,
    },
};

use super::data::AppState;

pub fn root_widget() -> impl Widget<AppState> {
    
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_flex_child(
            Icon::svg(&icon::BOWL_ICON)
                .with_scale(10.)
                .with_color(theme::COLOR_4), 1.0)
        .with_default_spacer()
        .with_child(Separator::new(5.));

    let screen = Flex::row()
        .with_flex_child(sidebar, 1.)
        .with_default_spacer()
        .with_child(Separator::new(5.).vertical(true).with_color(theme::COLOR_3))
        .with_default_spacer()
        .with_flex_child(Label::new("Body"), 1.61803);
    

    screen
}
