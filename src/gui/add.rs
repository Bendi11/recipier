//! Add recipe screen

use std::str::FromStr;

use druid::{Cursor, Widget, WidgetExt, text::format::ParseFormatter, theme, widget::{Container, Flex, LensWrap, List, RadioGroup, Scroll, Svg, SvgData, TextBox, ValueTextBox}};

use crate::{gui::state::{AddState, AppScreen, EditedIngredient}, recipes::{measure::{Mass, MassUnit, Volume, VolumeUnit}, recipe::IngredientAmount}};

use super::state::State;

/// Widget to add a recipe
pub fn add_recipe_widget() -> impl Widget<State> {
    let home_svg = SvgData::from_str(include_str!("../../assets/icons/home.svg")).unwrap();
    let home_button = Container::new(Svg::new(home_svg)
        .on_click(|ctx, state: &mut State, _env| {
            state.screen = AppScreen::View;
            ctx.set_handled()
        })
    )
    .fix_size(50., 50.);

    let layout = Flex::column()
        .with_spacer(10.)
        .with_child(Flex::row()
            .with_child(home_button)
            .with_flex_spacer(10.)
        );
    
    let name_text = LensWrap::new(TextBox::<String>::new()
        .with_placeholder("Recipe Name")
        .with_text_size(theme::TEXT_SIZE_LARGE)
        .border(theme::BORDER_DARK, 2.)
        .expand_width(),
        AddState::name
    )
    .lens(State::add_data)
    .align_left();

    let ingredient_name_editor = Container::new(
        TextBox::<String>::new()
            .with_placeholder("Ingredient Name")
            .with_text_size(theme::TEXT_SIZE_NORMAL)
        ).rounded(theme::TEXTBOX_BORDER_RADIUS)
        .border(theme::BORDER_DARK, theme::TEXTBOX_BORDER_WIDTH);

    
    let ingredient_amount_editor = Container::new(
        ValueTextBox::<f32>::new(
                TextBox::new()
                .with_placeholder("Ingredient Amount")
                .with_text_size(theme::TEXT_SIZE_NORMAL),
            ParseFormatter::new()
            ).validate_while_editing(false)
        ).rounded(theme::TEXTBOX_BORDER_RADIUS)
        .border(theme::BORDER_DARK, theme::TEXTBOX_BORDER_WIDTH);

    let ingredient_amount_unit_selector = RadioGroup::new([
        ("x", IngredientAmount::Count(0)),
        //Volume units
        ("cup", IngredientAmount::Volume(Volume::new(VolumeUnit::Cup, 0.))),
        ("liter", IngredientAmount::Volume(Volume::new(VolumeUnit::Liter, 0.))),
        ("milliliter", IngredientAmount::Volume(Volume::new(VolumeUnit::Milliliter, 0.))),
        ("tsp", IngredientAmount::Volume(Volume::new(VolumeUnit::Teaspoon, 0.))),
        ("tbsp", IngredientAmount::Volume(Volume::new(VolumeUnit::Tablespoon, 0.))),
        ("pint", IngredientAmount::Volume(Volume::new(VolumeUnit::Pint, 0.))),
        ("quart", IngredientAmount::Volume(Volume::new(VolumeUnit::Quart, 0.))),
        ("gallon", IngredientAmount::Volume(Volume::new(VolumeUnit::Gallon, 0.))),
        //Mass units
        ("gram", IngredientAmount::Mass(Mass::new(MassUnit::Gram, 0.))),
        ("kg", IngredientAmount::Mass(Mass::new(MassUnit::Kilogram, 0.))),
        ("mg", IngredientAmount::Mass(Mass::new(MassUnit::Milligram, 0.))),
        ("oz", IngredientAmount::Mass(Mass::new(MassUnit::Ounce, 0.))),
        ("lb", IngredientAmount::Mass(Mass::new(MassUnit::Pound, 0.))), 
    ]);

    let ingredients_editor = Container::new(
        Scroll::new(List::new(||
            Flex::row()
                .with_child(ingredient_name_editor.lens(EditedIngredient::name))
                .with_spacer(20.)
                .with_child(ingredient_amount_editor.lens(EditedIngredient::amt_num))
                .with_spacer(20.)
                .with_child(ingredient_amount_unit_selector.lens(EditedIngredient::amount))
        )).lens(AddState::ingredients)
    )
    
    layout
        .with_spacer(10.)
        .with_child(name_text)
}