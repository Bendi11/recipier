//! Widgets for displaying recipes

use std::sync::Arc;

use druid::{
    lens,
    widget::{Flex, Label, LineBreaking, List, Scroll},
    LensExt, TextAlignment, Widget, WidgetExt,
};

use crate::{gui::{CHANGE_SCREEN, EDIT_RECIPE, VIEW_RECIPE, data::{screen::AppScreen, AppState}, theme, widgets::{RecipierWidget, icon::{Icon, PEN_ICON, RIGHT_ARROW_ICON}, maybe::Maybe, separator::Separator}}, recipes::recipe::{Ingredient, Recipe}};

/// The string to use when formatting chrono datetimes
pub const DATETIME_FORMAT: &str = "%e %B %Y %I:%M";

/// Return a widget that displays one recipe in a maximized view
pub fn view_screen() -> impl Widget<AppState> {
    Maybe::or_empty(recipe_widget).lens(lens::Identity.map(
        |state: &AppState| state.recipes.get(state.view.viewed?),
        |state, recipe| {
            if let Some(recipe) = recipe {
                state.recipes.update(recipe);
            }
        },
    ))
}

/// Show a widget that displays all information about the recipe
pub fn recipe_widget() -> impl Widget<Arc<Recipe>> {
    Flex::column()
        .with_child(
            Label::raw()
                .with_font(theme::HEADER_FONT)
                .with_line_break_mode(LineBreaking::WordWrap)
                .align_left()
                .lens(Recipe::name),
        )
        .with_default_spacer()
        .with_child(
            Label::new(|recipe: &Recipe, _env: &'_ _| {
                format!("Created {}", recipe.created_on.format(DATETIME_FORMAT))
            })
            .align_left(),
        )
        .with_default_spacer()
        .with_child(Separator::new(2.))
        .with_default_spacer()
        .with_child(
            Label::new("Ingredients")
                .with_font(theme::LABEL_FONT)
                .align_left(),
        )
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(List::new(|| {
                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_child(Icon::svg(&RIGHT_ARROW_ICON).flex(false))
                            .with_spacer(3.)
                            .with_child(
                                Label::raw()
                                    .with_font(theme::SYSTEM_FONT)
                                    .lens(Ingredient::name)
                                    .align_left(),
                            )
                            .with_default_spacer()
                            .with_child(Label::new(|ingredient: &Ingredient, _env: &'_ _| {
                                format!("{}", ingredient.amount)
                            }))
                            .expand_width()
                            .padding((2.5, 5.))
                    )
                    .with_default_spacer()
                }).with_spacing(2.)
            )
            .vertical()
            .expand_width()
            .border(theme::COLOR_2, 2.)
            .rounded(5.0), 10.
        )
        .with_default_spacer()
        .with_flex_child(
            Label::raw()
                .with_font(theme::SYSTEM_FONT)
                .with_line_break_mode(LineBreaking::WordWrap)
                .with_text_alignment(TextAlignment::Start)
                .expand()
                .padding((5., 5.))
                .lens(Recipe::body), 30.
        )
        .expand()
        .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
        .padding((5., 1.))
}

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget() -> impl Widget<Recipe> {
    Flex::column()
        .with_child(Flex::row()
            .with_child(Label::raw()
                .with_font(theme::LABEL_FONT)
                .lens(Recipe::name)
                .align_left()
            )
            .with_flex_spacer(1.)
            .with_child(Icon::svg(&PEN_ICON)
                .on_hover(
                    |ctx, _data, this, _env| {
                        this.set_color(theme::COLOR_3);
                        ctx.request_paint();
                    }, 
                    |ctx, _data, this, _env| {
                        this.set_color(theme::COLOR_4);
                        ctx.request_paint();
                    }
                )
                .on_click(|ctx, recipe: &mut Recipe, _env| {
                    ctx.submit_command(EDIT_RECIPE.with(recipe.id));
                    ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Edit));
                })
            )
            .expand_width()
            .fix_height(25.)
        )
        .with_spacer(0.1)
        .with_child(Label::new(|data: &Recipe, _env: &'_ _| {
            format!("Created {}", data.created_on.format(DATETIME_FORMAT))
        }))
        .on_click(|ctx, recipe, _env| {
            ctx.submit_command(VIEW_RECIPE.with(recipe.id));
            ctx.submit_command(CHANGE_SCREEN.with(AppScreen::View));
        })
        .padding((2.5, 5.))
        .border(theme::COLOR_3, 2.)
        .rounded(7.)
        .on_hover(
            |ctx, _, this, _env| {
                this.set_background(theme::COLOR_2);
                ctx.request_paint();
            }, 
            |ctx, _, this, _env| {
                this.set_background(theme::COLOR_1);
                ctx.request_paint();
            }
        )
        .expand_width()
}
