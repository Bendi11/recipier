use std::str::FromStr;

use druid::{Widget, widget::{Flex, Label, Svg, SvgData}};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    let icon_data = SvgData::from_str(include_str!("../../../assets/icon.svg")).unwrap();
    let icon_svg = Svg::new(icon_data);
    let sidebar = Flex::column()
        .with_spacer(len)
}