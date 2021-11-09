use std::str::FromStr;

use druid::{Widget, WidgetExt, widget::{FillStrat, Flex, Label, Svg, SvgData}};

use crate::gui::{theme, widgets::separator::Separator};

use super::data::AppState;


pub fn root_widget() -> impl Widget<AppState> {
    let icon_data = SvgData::from_str(include_str!("../../../assets/icon.svg")).unwrap();
    let icon_svg = Svg::new(icon_data).fill_mode(FillStrat::Fill);
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_child(icon_svg.fix_size(100., 100.))
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