//! Widgets for the edit screen to change or create recipes

use druid::{Widget, WidgetExt, widget::{Flex, Label, TextBox}};

use crate::gui::{data::{AppState, edit::EditState}, theme, widgets::separator::Separator};


/// Build the root edit screen widget 
pub fn edit_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_child(Label::new("Edit")
            .with_font(theme::HEADER_FONT)
            .align_left()
            .expand_width()
        )
        .with_spacer(1.0)
        .with_child(Separator::new(2.0))
        .with_default_spacer()
        
        .with_child(Label::new("Title").with_font(theme::LABEL_FONT).align_left().expand_width())
        .with_spacer(1.0)
        .with_child(TextBox::new()
            .with_font(theme::SYSTEM_FONT)
            .expand_width()
            .padding((2.5, 5.))
            .border(theme::COLOR_2, 2.)
            .rounded(5.)
            .lens(EditState::title)
        )

        .lens(AppState::edit)
        .expand()
}
