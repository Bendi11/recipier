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
            .fix_height(50.)
        )
        .with_spacer(1.)
        .with_child(Separator::new(2.5).fix_width(130.).align_left())
        .with_default_spacer()
        
        .with_child(Label::new("Title").with_font(theme::LABEL_FONT).align_left().expand_width())
        .with_spacer(1.0)
        .with_child(TextBox::new()
            .with_font(theme::SYSTEM_FONT)
            .with_text_color(theme::COLOR_3)
            .with_text_size(17.)
            .expand_width()
            .padding((2.5, 0.))
            .lens(EditState::title)
        )

        .lens(AppState::edit)
        .expand()
}
