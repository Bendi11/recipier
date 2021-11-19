//! Widgets for the edit screen to change or create recipes

use druid::{TextAlignment, Widget, WidgetExt, text::format::ParseFormatter, widget::{Flex, Label, TextBox, ValueTextBox, ViewSwitcher}};

use crate::gui::{data::{AppState, edit::{EditState, EditedTime}}, theme, widgets::{icon::{Icon, PLUS_ICON, RECYCLE_ICON}, maybe::Maybe, separator::Separator}};


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
        .with_spacer(2.0)
        .with_child(TextBox::new()
            .with_font(theme::SYSTEM_FONT)
            .with_text_color(theme::COLOR_3)
            .with_text_size(17.)
            .expand_width()
            .padding((2.5, 0.))
            .lens(EditState::title)
        )
        .with_default_spacer()

        .with_child(Label::new("Time to Make").with_font(theme::LABEL_FONT).align_left().expand_width())
        .with_spacer(2.0)
        .with_child(ViewSwitcher::new(
            |data: &EditState, _env| data.time,
            |time, _data, _env| match time {
                Some(_) => Flex::row()
                    .with_child(Maybe::or_empty(|| Flex::row()
                        .with_child(ValueTextBox::new(TextBox::new().with_text_alignment(TextAlignment::Center).with_placeholder("hours"), ParseFormatter::new())
                            .validate_while_editing(true)
                            .fix_width(50.)
                            .lens(EditedTime::hours)
                        )
                        .with_spacer(5.0)
                        .with_child(ValueTextBox::new(TextBox::new().with_text_alignment(TextAlignment::Center).with_placeholder("minutes"), ParseFormatter::new())
                        .validate_while_editing(true)
                            .fix_width(50.)
                            .lens(EditedTime::minutes)
                        )
                        .with_spacer(5.0)
                        .with_child(ValueTextBox::new(TextBox::new().with_text_alignment(TextAlignment::Center).with_placeholder("seconds"), ParseFormatter::new())
                        .validate_while_editing(true)
                            .fix_width(50.)
                            .lens(EditedTime::secs)
                        )
                        .with_spacer(10.0)
                        
                    ).lens(EditState::time)
                )
                .with_child(Icon::svg(&RECYCLE_ICON)
                .highlight_on_hover()
                .on_click(|ctx, data: &mut EditState, _env| {
                    data.time = None;
                    ctx.request_update();
                })
                .fix_size(35., 35.)
                ).boxed(),
                None => Icon::svg(&PLUS_ICON)
                    .highlight_on_hover()
                    .on_click(|_ctx, data: &mut EditState, _env| {
                        data.time = Some(EditedTime::default())
                    })
                    .boxed()
            }, 
        ).expand_width().align_left().fix_height(50.)
        )
        .with_default_spacer()


        .lens(AppState::edit)
        .expand()
}
