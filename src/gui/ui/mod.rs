use druid::{Widget, widget::{Flex, Label}};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("test"))
}