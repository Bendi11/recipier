//! Module with function to build the GUI screens
pub mod state;

use std::str::FromStr;

use druid::{Widget, WidgetExt, theme, widget::{Align, Axis, Button, Flex, Label, Svg, SvgData, Tabs, TabsEdge, TabsTransition}};

use state::State;

use crate::gui::state::AppScreen;

/// Build the main screen widget
pub fn root_widget() -> impl Widget<State> {
    let tabs = Tabs::new()
        .with_edge(TabsEdge::Leading)
        .with_axis(Axis::Horizontal)
        .with_transition(TabsTransition::Slide(50_000))
        .with_tab("Recipes", recipes_widget());

    tabs
}

/// Build the recipes list widget
pub fn recipes_widget() -> impl Widget<State> {
    let mut layout = Flex::column();

    let plus_svg = match SvgData::from_str(include_str!("../../assets/icons/plus.svg")) {
        Ok(svg) => svg,
        Err(e) => {
            log::error!("Failed to parse SVG data from \"../../assets/icons/plus.svg\": {}", e);
            SvgData::empty()
        }
    };

    let plus_icon = Svg::new(plus_svg)
        .on_click(|ctx, state: &mut State, _env| {
            state.screen = AppScreen::Add {};
            ctx.set_handled()
        });

    let title_bar = Flex::row()
        .with_child(Label::new("Recipes")
            .with_text_size(theme::TEXT_SIZE_LARGE)
            .with_font(theme::UI_FONT_BOLD)
        )
        .with_default_spacer()
        .with_child(plus_icon);
        

    
    layout.add_child(title_bar);

    
    layout
}
