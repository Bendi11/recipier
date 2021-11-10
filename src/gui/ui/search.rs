//! Search widget builders that modify the search term state data and dispatch commands to search

use druid::{Data, Event, LensExt, Widget, WidgetExt, widget::{Controller, Flex, Label, Scroll, TextBox}};

use crate::gui::{CHANGE_SCREEN, POPULATE_RESULTS, data::{AppState, screen::AppScreen, 
    search::{Query, SearchResults,  SearchState}}, theme, widgets::{RecipierWidget, icon::{self, Icon}, maybe::Maybe, separator::Separator}};

/// Widget controller that sends a navigate to search results command when the enter key is pressed
struct EnterController;

impl<D: Data, W: Widget<D>> Controller<D, W> for EnterController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut D, env: &druid::Env) {
        if ctx.is_focused() {
            match event {
                Event::KeyDown(key) if 
                    key.code == druid::keyboard_types::Code::Enter || 
                    key.code == druid::keyboard_types::Code::NumpadEnter => {
                        ctx.submit_command(POPULATE_RESULTS);
                        ctx.submit_command(CHANGE_SCREEN.with(AppScreen::SearchResults))
                }
                _ => ()
            }
        }
        child.event(ctx, event, data, env)
    }
}

/// Generate the root search results widget
pub fn search_screen() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_flex_child(search_bar().lens(AppState::search.then(SearchState::query.then(Query::term))), 10.)
        .with_default_spacer()
        .with_child(Separator::new(1.0))
        .with_child(Maybe::or_empty(|| Flex::row()
                .with_child(Label::new("Results for").with_font(theme::SMALL_FONT))
                .with_child(Label::raw().with_font(theme::SMALL_FONT).lens(SearchResults::term))
            )
            .lens(AppState::search.then(SearchState::results))
            .align_left()
        )
        .with_flex_child(Maybe::or_empty(|| Scroll::new(
                Label::new("results") 
            )).lens(AppState::search.then(SearchState::results)), 100.)
}

/// Return a search bar that modifies a search term string and sends the change screen
/// command on enter
pub fn search_bar() -> impl Widget<String> {
    Flex::row()
        .with_flex_child(TextBox::new()
            .with_text_color(theme::COLOR_3)
            .with_placeholder("Search")
            .controller(EnterController)
            .expand_width(),
            2.
        )
        .with_flex_spacer(0.1)
        .with_flex_child(Icon::svg(&icon::SEARCH_ICON).with_color(theme::COLOR_3).on_hover(|ctx, _d, this, _env| {
            this.set_color(theme::COLOR_4); 
            ctx.request_paint();
        }, |ctx, _data, this, _env| {
            this.set_color(theme::COLOR_3);
            ctx.request_paint();
        }).on_click(|ctx, _data, _env| {
            ctx.submit_command(POPULATE_RESULTS);
            ctx.submit_command(CHANGE_SCREEN.with(AppScreen::SearchResults));
        }), 0.4)
}