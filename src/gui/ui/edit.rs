//! Widgets for the edit screen to change or create recipes

use druid::{TextAlignment, Widget, WidgetExt, text::format::Validation, widget::{Button, Flex, Label, List, Scroll, TextBox, ValueTextBox, ViewSwitcher}};

use crate::gui::{data::{AppState, edit::{EditState, EditedIngredient, EditedTime}}, theme, widgets::{icon::{Icon, PLUS_ICON, RECYCLE_ICON}, maybe::Maybe, separator::Separator, unit::UnitSelectorController}};

/// Build the root edit screen widget
pub fn edit_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_child(
            Label::new("Edit")
                .with_font(theme::HEADER_FONT)
                .align_left()
                .expand_width()
                .fix_height(50.),
        )
        .with_spacer(1.)
        .with_child(Separator::new(2.5).fix_width(130.).align_left())
        .with_default_spacer()
        .with_child(
            Label::new("Title")
                .with_font(theme::LABEL_FONT)
                .align_left()
                .expand_width(),
        )
        .with_spacer(2.0)
        .with_child(
            TextBox::new()
                .with_font(theme::SYSTEM_FONT)
                .with_text_color(theme::COLOR_3)
                .with_text_size(17.)
                .expand_width()
                .padding((2.5, 0.))
                .lens(EditState::title),
        )
        .with_default_spacer()
        .with_child(
            Label::new("Time to Make")
                .with_font(theme::LABEL_FONT)
                .align_left()
                .expand_width(),
        )
        .with_spacer(2.0)
        .with_child(time_editor())
        .with_default_spacer()
        .with_child(Label::new("Ingredients").with_font(theme::HEADER_FONT).align_left().expand_width())
        .with_default_spacer()
        .with_child(Scroll::new(List::new(ingredient_editor

        )).vertical().fix_height(300.))
        .lens(AppState::edit)
        .expand()
}



/// Build an ingredient editor for 
fn ingredient_editor() -> impl Widget<EditedIngredient> {
    Flex::row()
        .with_child(TextBox::new().with_placeholder("Ingredient name").align_left().lens(EditedIngredient::name))
        .with_spacer(10.)
        .with_child(ValueTextBox::new(TextBox::new().with_placeholder("Amount"), FloatEditorFormatter).lens(EditedIngredient::count))
        .with_spacer(5.)
        .with_child(Button::dynamic(|ingredient: &EditedIngredient, _env| ingredient.amount.to_string()).controller(UnitSelectorController) )
        
}


/// Build the root widget for the recipe time editor 
fn time_editor() -> impl Widget<EditState> {
    ViewSwitcher::new(
        |data: &EditState, _env| data.time,
        |time, _data, _env| match time {
            Some(_) => Flex::row()
                .with_child(
                    Maybe::or_empty(|| {
                        Flex::row()
                            .with_child(Label::new("Hr").with_font(theme::SMALL_FONT))
                            .with_spacer(5.0)
                            .with_child(
                                ValueTextBox::new(
                                    TextBox::new()
                                        .with_text_alignment(TextAlignment::Center)
                                        .with_placeholder("hours"),
                                        NumberEditorFormatter,
                                )
                                .fix_width(50.)
                                .lens(EditedTime::hours),
                            )
                            .with_spacer(10.0)
                            .with_child(Label::new("Min").with_font(theme::SMALL_FONT))
                            .with_spacer(5.0)
                            .with_child(
                                ValueTextBox::new(
                                    TextBox::new()
                                        .with_text_alignment(TextAlignment::Center)
                                        .with_placeholder("minutes"),
                                        NumberEditorFormatter,
                                )
                                .fix_width(50.)
                                .lens(EditedTime::minutes),
                            )
                            .with_spacer(10.0)
                            .with_child(Label::new("Sec").with_font(theme::SMALL_FONT))
                            .with_spacer(5.0)
                            .with_child(
                                ValueTextBox::new(
                                    TextBox::new()
                                        .with_text_alignment(TextAlignment::Center)
                                        .with_placeholder("seconds"),
                                    NumberEditorFormatter,
                                )
                                
                                .fix_width(50.)
                                .lens(EditedTime::secs),
                            )
                            .with_spacer(20.0)
                    })
                    .lens(EditState::time),
                )
                .with_child(
                    Icon::svg(&RECYCLE_ICON)
                        .highlight_on_hover()
                        .on_click(|ctx, data: &mut EditState, _env| {
                            data.time = None;
                            ctx.request_update();
                        })
                        .fix_size(35., 35.),
                )
                .boxed(),
            None => Icon::svg(&PLUS_ICON)
                .highlight_on_hover()
                .on_click(|_ctx, data: &mut EditState, _env| {
                    data.time = Some(EditedTime::default())
                })
                .boxed(),
        },
    )
    .expand_width()
    .align_left()
    .fix_height(50.)
}

/// A structure implementing [Formatter](druid::text::format::Formatter) to parse a `u8` from the input box
#[derive(Clone, Copy, Debug)]
struct NumberEditorFormatter;
impl druid::text::format::Formatter<u8> for NumberEditorFormatter {
    fn format(&self, value: &u8) -> String {
        value.to_string()
    }

    fn format_for_editing(&self, value: &u8) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &druid::text::Selection) -> Validation {
        if input[sel.range()].is_empty() {
            Validation::success()
        } else {
            match input[sel.range()].parse::<u8>() {
                Ok(_) => Validation::success(),
                Err(e) => Validation::failure(e)
            }
        }
    }

    fn value(&self, input: &str) -> Result<u8, druid::text::format::ValidationError> {
        if input.is_empty() {
            return Ok(0)
        }
        input.parse().map_err(|e| druid::text::format::ValidationError::new(e))
    }
}

/// A structure implementing [Formatter](druid::text::format::Formatter) to parse a `f32` from the input box
#[derive(Clone, Copy, Debug)]
struct FloatEditorFormatter;
impl druid::text::format::Formatter<f32> for FloatEditorFormatter {
    fn format(&self, value: &f32) -> String {
        value.to_string()
    }

    fn format_for_editing(&self, value: &f32) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, sel: &druid::text::Selection) -> Validation {
        if input[sel.range()].is_empty() {
            Validation::success()
        } else {
            match input[sel.range()].parse::<f32>() {
                Ok(_) => Validation::success(),
                Err(e) => Validation::failure(e)
            }
        }
    }

    fn value(&self, input: &str) -> Result<f32, druid::text::format::ValidationError> {
        if input.is_empty() {
            return Ok(0f32)
        }
        input.parse().map_err(|e| druid::text::format::ValidationError::new(e))
    }
}
