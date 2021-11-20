pub mod edit;
pub mod home;
pub mod recipe;
pub mod remove;
pub mod search;

use druid::{
    widget::{Flex, ViewSwitcher},
    LensExt, Widget, WidgetExt,
};

use crate::gui::{
    theme,
    widgets::{
        icon::{self, Icon},
        separator::Separator,
    },
};

use super::{
    data::{
        screen::AppScreen,
        search::{Query, SearchState},
        AppState,
    },
    widgets::RecipierWidget,
    CHANGE_SCREEN,
};

pub fn root_widget() -> impl Widget<AppState> {
    let screen = ViewSwitcher::new(
        |state: &AppState, _env| state.screen,
        |screen, _state, _env| match screen {
            AppScreen::Delete => remove::remove_widget().boxed(),
            AppScreen::Edit => edit::edit_widget().boxed(),
            AppScreen::Home => home::home_widget().boxed(),
            AppScreen::SearchResults => search::search_screen().boxed(),
            AppScreen::View => recipe::view_screen().boxed(),
        },
    )
    .expand();

    screen
}

/// Build the sidebar with search and home icon
pub fn sidebar() -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_spacer(theme::SPACING)
                .with_child(
                    Icon::svg(&icon::BOWL_ICON)
                        .with_scale(10.)
                        .flex(false)
                        .with_color(theme::COLOR_4)
                        .on_hover(
                            |ctx, _, this, _| {
                                this.set_scale(10.5);
                                this.set_color(theme::COLOR_3);
                                ctx.request_layout();
                                ctx.request_paint();
                            },
                            |ctx, _, this, _| {
                                this.set_scale(10.);
                                this.set_color(theme::COLOR_4);
                                ctx.request_layout();
                                ctx.request_paint();
                            },
                        )
                        .on_click(|ctx, _data, _env| {
                            ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Home))
                        }),
                )
                .with_child(Separator::new(5.).with_ratio(1.))
                .with_default_spacer()
                .with_child(
                    search::search_bar()
                        .lens(AppState::search.then(SearchState::query).then(Query::term)),
                )
                .with_default_spacer()
                .with_flex_spacer(0.5)
                .padding((5., 0., 0., 0.))
                .fix_width(200.),
        )
        .with_default_spacer()
        .with_child(Separator::new(5.).vertical(true).with_color(theme::COLOR_2))
        .with_default_spacer()
}
