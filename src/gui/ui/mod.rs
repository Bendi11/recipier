use std::str::FromStr;

use druid::{Widget, widget::{FillStrat, Flex, Label, Svg, SvgData}};

use crate::gui::{theme, widgets::separator::Separator};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    let icon_data = SvgData::from_str(include_str!("../../../assets/icon.svg")).unwrap();
    let icon_svg = Svg::new(icon_data).fill_mode(FillStrat::Contain);
    /*let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_child(Label::new("Test"))
        .with_child(Separator::new(10.))
        .with_child(Label::new("Test"));
    
    let screen = Flex::row()
        .with_child(sidebar);*/
    
    icon_svg
}