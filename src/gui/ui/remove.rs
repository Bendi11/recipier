//! Remove confirmation dialog screen

use std::sync::Arc;

use druid::{
    widget::{Button, Flex, Label},
    Widget, WidgetExt,
};

use crate::{
    gui::{
        data::{remove::RemoveState, screen::AppScreen, AppState},
        theme,
        widgets::{maybe::Maybe, separator::Separator},
        CHANGE_SCREEN,
    },
    recipes::recipe::Recipe,
};

/// Build the root widget for the recipe removal confirmation screen
pub fn remove_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Maybe::new(
            || Flex::column()
                .with_default_spacer()
                .with_child(Label::dynamic(|data: &Arc<Recipe>, _| format!("Delete {}?", data.name)).with_font(theme::HEADER_FONT))
                .with_spacer(1.)
                .with_child(Separator::new(2.5).fix_width(130.))
                .with_default_spacer()
                .lens(RemoveState::deleted),
            || Label::dynamic(|_, _| {
                log::error!("Remove screen displayed while there is no recipe to remove!");
                "Internal Error, please return to home screen and report error to bkliebmann@gmail.com".to_owned()
            })
        )
        .lens(AppState::remove)
    )
    .with_child(Flex::row()
        .with_child(Button::new("Cancel")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(ref remove) = data.remove {
                    ctx.submit_command(CHANGE_SCREEN.with(remove.return_to))
                }
                data.remove = None;
            })
        )
        .with_flex_spacer(1.0)
        .with_child(Button::new("Delete")
            .on_click(|ctx, data: &mut AppState, _env| {
                if let Some(ref remove) = data.remove {
                    data.recipes.remove(remove.deleted.id);
                    ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Home)) //Ignore the return to option because we deleted the recipe
                }
                data.remove = None;
            })
        )
        .padding((10., 0.))
    )
}
