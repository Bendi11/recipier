//! Widgets for displaying recipes

use std::{sync::Arc, time::Duration};

use druid::{
    lens,
    widget::{FillStrat, Flex, Image, Label, LineBreaking, List, Scroll, SizedBox},
    Data, LensExt, LifeCycle, TextAlignment, Widget, WidgetExt,
};

use crate::{
    gui::{
        data::{screen::AppScreen, AppState},
        theme,
        widgets::{
            icon::{PEN_ICON, RECYCLE_ICON, RIGHT_ARROW_ICON},
            maybe::Maybe,
            separator::Separator,
            RecipierWidget,
        },
        CHANGE_SCREEN, EDIT_RECIPE, REMOVE_RECIPE, VIEW_RECIPE,
    },
    recipes::recipe::{Ingredient, Recipe},
};

use super::sidebar;

/// The string to use when formatting chrono datetimes
pub const DATETIME_FORMAT: &str = "%e %B %Y %I:%M";

/// Return a widget that displays one recipe in a maximized view
pub fn view_screen() -> impl Widget<AppState> {
    Flex::row()
        .with_child(sidebar())
        .with_flex_child(Scroll::new(recipe_widget()).vertical().expand_height(), 1.0)
}

/// Show a widget that displays all information about the recipe
pub fn recipe_widget() -> impl Widget<AppState> {
    let recipe_lens = lens::Identity.map(
        |state: &AppState| state.recipes.get(state.view.viewed?),
        |state, recipe| {
            if let Some(recipe) = recipe {
                state.recipes.update(recipe);
            }
        },
    );

    let top = Maybe::or_empty(|| {
        Flex::column()
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(
                        Label::raw()
                            .with_font(theme::HEADER_FONT)
                            .with_line_break_mode(LineBreaking::WordWrap)
                            .align_left()
                            .lens(Recipe::name),
                    )
                    .with_flex_spacer(1.)
                    .with_child(
                        Flex::column()
                            .with_child(edit_button(AppScreen::View))
                            .with_flex_spacer(1.0)
                            .with_child(delete_button(AppScreen::View)),
                    )
                    .fix_height(50.),
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
            .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
    });

    let image = ImageBuilder::new().fix_height(175.);

    let lower = Maybe::or_empty(|| {
        Flex::column()
            .with_spacer(5.0)
            .with_child(
                Maybe::or_empty(|| {
                    Flex::column()
                        .with_child(
                            Label::new(|servings: &f32, _env: &'_ _| {
                                format!(
                                    "Makes {} serving{}",
                                    servings,
                                    if *servings == 1f32 { "" } else { "s" }
                                )
                            })
                            .with_font(theme::SYSTEM_FONT)
                            .expand_width(),
                        )
                        .with_default_spacer()
                })
                .lens(Recipe::servings),
            )
            .with_child(
                Maybe::or_empty(|| {
                    Flex::column()
                        .with_child(
                            Label::new(|time: &f32, _env: &'_ _| {
                                format!("Takes {} to cook", FormattedDuration(*time))
                            })
                            .align_left()
                            .expand_width(),
                        )
                        .with_default_spacer()
                })
                .lens(Recipe::time.map(
                    |duration| duration.map(|v| v.as_secs_f32()),
                    |duration, seconds| {
                        if let Some(seconds) = seconds {
                            *duration = Some(Duration::from_secs_f32(seconds));
                        }
                    },
                )),
            )
            .with_child(
                Label::new("Ingredients")
                    .with_font(theme::LABEL_FONT)
                    .align_left(),
            )
            .with_default_spacer()
            .with_child(
                List::new(|| {
                    Flex::column()
                        .with_child(
                            Flex::row()
                                .with_child(RIGHT_ARROW_ICON.clone().flex(false))
                                .with_spacer(3.)
                                .with_child(
                                    Label::raw()
                                        .with_font(theme::SYSTEM_FONT)
                                        .with_line_break_mode(LineBreaking::WordWrap)
                                        .lens(Ingredient::name)
                                        .align_left(),
                                )
                                .with_default_spacer()
                                .with_child(Label::new(|ingredient: &Ingredient, _env: &'_ _| {
                                    format!("{}", ingredient.amount)
                                }))
                                .expand_width()
                                .padding((2.5, 5.)),
                        )
                        .with_default_spacer()
                })
                .with_spacing(2.)
                .expand_width()
                .border(theme::COLOR_2, 2.)
                .rounded(5.0),
            )
            .with_default_spacer()
            .with_child(
                Label::raw()
                    .with_font(theme::SYSTEM_FONT)
                    .with_text_size(16.)
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .with_text_alignment(TextAlignment::Start)
                    .expand_width()
                    .padding((5., 5.))
                    .lens(Recipe::body),
            )
            .expand_width()
            .padding((15., 0.))
            .lens(LensExt::<Arc<Recipe>, Arc<Recipe>>::in_arc(lens::Identity))
    });

    Flex::column()
        .with_child(top.lens(recipe_lens.clone()))
        .with_child(image)
        .with_child(lower.lens(recipe_lens))
        .padding((0., 0., 10., 0.))
}

/// A remove recipe button that takes the user to a confirmation dialog
fn delete_button(screen: AppScreen) -> impl Widget<Recipe> {
    RECYCLE_ICON
        .clone()
        .highlight_on_hover()
        .on_click(move |ctx, recipe: &mut Recipe, _env| {
            ctx.submit_command(REMOVE_RECIPE.with((recipe.id, screen)));
            ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Delete));
        })
        .fix_size(20., 20.)
}

/// Edit icon button that takes the user to the edit screen populated with the current recipe's data
fn edit_button(screen: AppScreen) -> impl Widget<Recipe> {
    PEN_ICON
        .clone()
        .highlight_on_hover()
        .on_click(move |ctx, recipe: &mut Recipe, _env| {
            ctx.submit_command(EDIT_RECIPE.with((recipe.id, screen)));
            ctx.submit_command(CHANGE_SCREEN.with(AppScreen::Edit));
        })
        .fix_size(20., 20.)
}

/// Show a peek of a recipe with brief details
pub fn recipe_brief_widget() -> impl Widget<Recipe> {
    let recipe = Flex::column()
        .with_child(
            Label::raw()
                .with_font(theme::LABEL_FONT)
                .with_line_break_mode(LineBreaking::WordWrap)
                .lens(Recipe::name)
                .align_left(),
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
            },
        )
        .expand_width();

    Flex::row()
        .with_flex_child(recipe, 10.)
        .with_spacer(5.0)
        .with_child(
            Flex::column()
                .with_child(edit_button(AppScreen::Home))
                .with_spacer(5.5)
                .with_child(delete_button(AppScreen::Home)),
        )
        .with_spacer(5.0)
        .expand_width()
}

/// A newtype over [f32] used for a custom [Display](std::fmt::Display) impl that shows
/// the duration in a more readable way
struct FormattedDuration(f32);
impl std::fmt::Display for FormattedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MILLIS_DAY: f32 = 86400f32;
        const MILLIS_HOUR: f32 = 3600f32;
        const MILLIS_MINUTE: f32 = 60f32;
        const MILLIS_SECOND: f32 = 1f32;

        let seconds = self.0;
        if seconds >= MILLIS_DAY {
            write!(f, "{} day", seconds / MILLIS_DAY)?;
            if seconds >= (MILLIS_DAY * 2f32) {
                write!(f, "s")?;
            }
        } else if seconds >= MILLIS_HOUR {
            write!(f, "{} hour", seconds / MILLIS_HOUR)?;
            if seconds >= (MILLIS_HOUR * 2f32) {
                write!(f, "s")?;
            }
        } else if seconds >= MILLIS_MINUTE {
            write!(f, "{} min", seconds / MILLIS_MINUTE)?;
            if seconds >= (MILLIS_MINUTE * 2f32) {
                write!(f, "s")?;
            }
        } else if seconds >= MILLIS_SECOND {
            write!(f, "{} sec", seconds / MILLIS_SECOND)?;
            if seconds >= (MILLIS_SECOND * 2f32) {
                write!(f, "s")?;
            }
        } else {
            write!(f, "{} seconds", seconds)?;
        }

        Ok(())
    }
}

/// A widget that builds an image using app state
struct ImageBuilder {
    /// The internal widget to display
    widget: Box<dyn Widget<AppState>>,
}

impl ImageBuilder {
    /// Create a new empty image builder
    pub fn new() -> Self {
        Self {
            widget: SizedBox::empty().boxed(),
        }
    }
}

impl Widget<AppState> for ImageBuilder {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        self.widget.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            if let Some(id) = data.view.viewed {
                if let Some(data) = data.recipes.get_image(id).as_ref() {
                    self.widget = Image::new(data.clone())
                        .fill_mode(FillStrat::Contain)
                        .boxed();
                }
            }
        }
        self.widget.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        _env: &druid::Env,
    ) {
        if !old_data.same(&data) {
            if let Some(id) = data.view.viewed {
                if let Some(data) = data.recipes.get_image(id).as_ref() {
                    self.widget = Image::new(data.clone())
                        .fill_mode(FillStrat::Contain)
                        .boxed();
                    ctx.children_changed()
                }
            }
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &druid::Env,
    ) -> druid::Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.widget.paint(ctx, data, env)
    }

    fn id(&self) -> Option<druid::WidgetId> {
        self.widget.id()
    }
}
