//! Add recipe screen

use std::str::FromStr;

use druid::{Widget, WidgetExt, theme, widget::{Container, Flex, LensWrap, Svg, SvgData, TextBox}};

use crate::gui::state::{AddState, AppScreen};

use super::state::State;

/// Widget to add a recipe
pub fn add_recipe_widget() -> impl Widget<State> {
    let home_svg = SvgData::from_str(include_str!("../../assets/icons/home.svg")).unwrap();
    let home_button = Container::new(Svg::new(home_svg)
        .on_click(|ctx, state: &mut State, _env| {
            state.screen = AppScreen::View;
            ctx.set_handled()
        })
    )
    .fix_size(50., 50.);

    let layout = Flex::column()
        .with_spacer(10.)
        .with_child(Flex::row()
            .with_child(home_button)
            .with_flex_spacer(10.)
        );
    
    let name_text = LensWrap::new(TextBox::<String>::new()
        .with_placeholder("Recipe Name")
        .with_text_size(theme::TEXT_SIZE_LARGE)
        .border(theme::BORDER_DARK, 2.)
        .expand_width(),
        AddState::name
    )
    .lens(State::add_data)
    .align_left();

    layout
        .with_spacer(10.)
        .with_child(name_text)
}