//! Search widget builders that modify the search term state data and dispatch commands to search

use std::sync::Arc;

use druid::{
    lens,
    widget::{Controller, Flex, Label, List, Scroll, SizedBox, TextBox},
    Data, Event, LensExt, Widget, WidgetExt,
};

use crate::{
    gui::{
        data::{
            screen::AppScreen,
            search::{Query, SearchResults, SearchState},
            AppState,
        },
        theme,
        widgets::{icon, maybe::Maybe, separator::Separator, RecipierWidget},
        CHANGE_SCREEN, POPULATE_RESULTS,
    },
    recipes::recipe::Recipe,
};

use super::{recipe::recipe_brief_widget, sidebar};

/// Widget controller that sends a navigate to search results command when the enter key is pressed
struct EnterController;

impl<D: Data, W: Widget<D>> Controller<D, W> for EnterController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut D,
        env: &druid::Env,
    ) {
        if ctx.is_focused() {
            match event {
                Event::KeyDown(key)
                    if key.code == druid::keyboard_types::Code::Enter
                        || key.code == druid::keyboard_types::Code::NumpadEnter =>
                {
                    ctx.submit_command(POPULATE_RESULTS);
                    ctx.submit_command(CHANGE_SCREEN.with(AppScreen::SearchResults))
                }
                _ => (),
            }
        }
        child.event(ctx, event, data, env)
    }
}

/// Generate the root search results widget
pub fn search_screen() -> impl Widget<AppState> {
    Flex::row().with_child(sidebar()).with_flex_child(
        Flex::column()
            .with_default_spacer()
            .with_child(
                search_bar().lens(AppState::search.then(SearchState::query.then(Query::term))),
            )
            .with_default_spacer()
            .with_child(
                Maybe::or_empty(|| {
                    Flex::row()
                        .with_child(Label::new("Results for").with_font(theme::SMALL_FONT))
                        .with_child(
                            Label::raw()
                                .with_font(theme::SMALL_FONT)
                                .lens(SearchResults::term),
                        )
                        .align_left()
                })
                .lens(AppState::search.then(SearchState::results)),
            )
            .with_child(Separator::new(1.0))
            .with_default_spacer()
            .with_flex_child(
                Maybe::new(
                    || {
                        Scroll::new(
                            List::new(|| {
                                recipe_brief_widget().lens(
                                    LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity),
                                )
                            })
                            .with_spacing(10.),
                        )
                        .vertical()
                    },
                    || SizedBox::empty().expand_height(),
                )
                .expand_width()
                .lens(AppState::search.then(SearchState::results)),
                10.,
            )
            .padding((2., 0.)),
        1.0,
    )
}

/// Return a search bar that modifies a search term string and sends the change screen
/// command on enter
pub fn search_bar() -> impl Widget<String> {
    Flex::row()
        .with_flex_child(
            TextBox::new()
                .with_text_color(theme::COLOR_3)
                .with_placeholder("Search")
                .controller(EnterController)
                .expand_width(),
            2.,
        )
        .with_flex_spacer(0.1)
        .with_flex_child(
            icon::SEARCH_ICON
                .clone()
                .with_color(theme::COLOR_3)
                .on_hover(
                    |ctx, _d, this, _env| {
                        this.set_color(theme::COLOR_4);
                        ctx.request_paint();
                    },
                    |ctx, _data, this, _env| {
                        this.set_color(theme::COLOR_3);
                        ctx.request_paint();
                    },
                )
                .on_click(|ctx, _data, _env| {
                    ctx.submit_command(POPULATE_RESULTS);
                    ctx.submit_command(CHANGE_SCREEN.with(AppScreen::SearchResults));
                }),
            0.3,
        )
        .fix_height(50.)
}
