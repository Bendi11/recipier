//! Module with function to build the GUI screens
pub mod state;
pub mod add;

use std::str::FromStr;

use druid::{Widget, WidgetExt, theme, widget::{Axis, Container, Flex, Label, Svg, SvgData, Tabs, TabsEdge, TabsTransition, ViewSwitcher}};

use state::State;

use crate::gui::state::AppScreen;

/// Build the main screen widget
pub fn root_widget() -> impl Widget<State> {
    let switcher = ViewSwitcher::<State, _>::new(
        |state, _env| state.screen,
        |screen, _data, _env| match screen {
            AppScreen::Add => {
                Box::new(Label::new("test"))
            },
            AppScreen::View => Box::new(recipes_widget())
        }
    );

    switcher
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

    let plus_icon = Container::new(Svg::new(plus_svg)
        .on_click(|ctx, state: &mut State, _env| {
            state.screen = AppScreen::Add;
            ctx.set_handled()
        })
    )
    .fix_size(25., 25.);

    let title_bar = Flex::row()
        .with_child(Label::new("Recipes")
            .with_text_size(theme::TEXT_SIZE_LARGE)
            .with_font(theme::UI_FONT_BOLD)
        )
        .with_flex_spacer(100.)
        .with_child(plus_icon);
        

    
    layout.add_child(title_bar);

    
    layout
}
