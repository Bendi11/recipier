
use druid::{Widget, widget::{Flex, Label}};

use crate::gui::{theme, widgets::{icon::{self, Icon}, separator::Separator}};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_child(Icon::svg(&icon::BOWL_ICON).with_scale(10.).with_color(theme::COLOR_3))
        .with_child(Label::new("Test"))
        .with_default_spacer()
        .with_child(Separator::new(5.))
        .with_default_spacer()
        .with_child(Label::new("Test"))
        .with_default_spacer()
        .with_child(Separator::new(5.))
        .with_default_spacer();
    
    sidebar
}