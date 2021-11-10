//! Search widget builders that modify the search term state data and dispatch commands to search

use druid::{Data, Event, Widget, WidgetExt, widget::{Controller, TextBox}};

use crate::gui::{CHANGE_SCREEN, data::screen::AppScreen, theme};

/// Widget controller that sends a navigate to search results command when the enter key is pressed
struct EnterController;

impl<D: Data, W: Widget<D>> Controller<D, W> for EnterController {
    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut D, env: &druid::Env) {
        if ctx.is_focused() {
            match event {
                Event::KeyDown(key) if 
                    key.code == druid::keyboard_types::Code::Enter || 
                    key.code == druid::keyboard_types::Code::NumpadEnter => {
                        ctx.submit_command(CHANGE_SCREEN.with(AppScreen::SearchResults))
                }
                _ => ()
            }
        }
        child.event(ctx, event, data, env)
    }
}

/// Return a search bar that modifies a search term string and sends the change screen
/// command on enter
pub fn search_bar() -> impl Widget<String> {
    TextBox::new()
        .with_text_color(theme::COLOR_2)
        .with_placeholder("Search")
        .controller(EnterController)
        .border(theme::COLOR_2, 1.0)
        .rounded(25.)
}