use std::str::FromStr;

use druid::{Widget, widget::{Flex, Label, Svg, SvgData}};

use crate::gui::{theme, widgets::separator::Separator};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    let icon_data = SvgData::from_str(include_str!("../../../assets/icon.svg")).unwrap();
    let icon_svg = Svg::new(icon_data);
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_child(icon_svg)
        .with_flex_spacer(1.0)
        .with_child(Separator::new(1.));
    
    let screen = Flex::row()
        .with_child(sidebar);
    
    screen
}