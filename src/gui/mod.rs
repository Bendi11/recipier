//! Module with function to build the GUI screens
pub mod state;
pub mod add;

use std::str::FromStr;

use druid::{Widget, WidgetExt, theme, widget::{Container, Flex, Label, List, Scroll, Svg, SvgData, ViewSwitcher}};

use state::State;
use uuid::Uuid;

use crate::{gui::state::AppScreen, recipes::{db::RecipeId, recipe::Recipe}};

/// Build the main screen widget
pub fn root_widget() -> impl Widget<State> {
    let switcher = ViewSwitcher::<State, _>::new(
        |state, _env| state.screen,
        |screen, _data, _env| match screen {
            AppScreen::Add => {
                let home_svg = SvgData::from_str(include_str!("../../assets/icons/home.svg")).unwrap();
                let home_button = Container::new(Svg::new(home_svg)
                    .on_click(|ctx, state: &mut State, _env| {
                        state.screen = AppScreen::View;
                        ctx.set_handled()
                    })
                )
                .fix_size(25., 25.);

                let layout = Flex::column()
                    .with_flex_spacer(10.)
                    .with_child(Flex::row()
                        .with_child(home_button)
                        .with_flex_spacer(10.)
                    );

                Box::new(layout)
            },
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
    .fix_size(25., 25.);

    let title_bar = Flex::row()
        .with_child(Label::new("Recipes")
            .with_text_size(theme::TEXT_SIZE_LARGE)
            .with_font(theme::UI_FONT_BOLD)
        )
        .with_flex_spacer(100.)
        .with_child(plus_icon);
        
    let list = Scroll::new(List::new(|| {
        Label::new(|item: &Recipe, _env: &'_ _| {
            item.name.clone()
        })
        
    })).lens(State::recipes);
    
    layout.add_child(title_bar);
    layout.add_spacer(10.);
    layout.add_child(list);

    
    layout
}
