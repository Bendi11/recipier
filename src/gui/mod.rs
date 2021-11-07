//! Module with function to build the GUI screens
pub mod state;
pub mod add;

use std::str::FromStr;

use druid::{Widget, WidgetExt, text::RichText, theme, widget::{Container, Flex, Label, List, Scroll, Svg, SvgData, ViewSwitcher}};

use state::State;

use crate::{gui::state::AppScreen, recipes:: recipe::Recipe};

use self::add::add_recipe_widget;

/// Build the main screen widget
pub fn root_widget() -> impl Widget<State> {
    let switcher = ViewSwitcher::<State, _>::new(
        |state, _env| state.screen,
        |screen, _data, _env| match screen {
            AppScreen::Add => Box::new(add_recipe_widget()),
            AppScreen::View => Box::new(recipes_widget())
        }
    );

    switcher
}

/// Build the recipes list widget
pub fn recipes_widget() -> impl Widget<State> {
    let mut layout = Flex::column()
        .with_spacer(10.);

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
    .fix_size(50., 50.);

    let title_bar = Flex::row()
        .with_flex_spacer(100.)
        .with_child(plus_icon)
        .with_spacer(10.);
        
    let list = Scroll::new(List::new(|| {
        Flex::column()
            .with_child(Label::new(|recipe: &Recipe, _env: &'_ _| RichText::)
                .align_left()
                
            )

            .expand_width()
            .border(theme::BORDER_DARK, 2.)
        
    }))
    .vertical()
    .lens(State::recipes);
    
    layout.add_child(title_bar);
    layout.add_spacer(10.);
    layout.add_child(list);

    
    layout
}
