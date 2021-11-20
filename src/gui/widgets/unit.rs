//! Widget to display a dropdown used to select an ingredient amount

use druid::{widget::Controller, ContextMenu, Event, LocalizedString, MenuDesc, MenuItem, Widget};

use crate::gui::{
    data::{edit::EditedIngredient, AppState},
    CHANGE_INGREDIENT_UNIT,
};

/// Widget allowing the user to modify the units of an ingredient amount
pub struct UnitSelectorController;

impl<W: Widget<EditedIngredient>> Controller<EditedIngredient, W> for UnitSelectorController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut EditedIngredient,
        env: &druid::Env,
    ) {
        use crate::recipes::measure::*;

        macro_rules! unit_item {
            ($unit:expr, $name:literal) => {
                MenuItem::new(
                    LocalizedString::new($name),
                    CHANGE_INGREDIENT_UNIT.with((data.id, $unit)),
                )
            };
        }

        let unit_selector = MenuDesc::<AppState>::new(LocalizedString::new("Unit"))
            .append(unit_item!(AmountUnit::Count, "count"))
            .append(unit_item!(AmountUnit::None, "no measure"))
            .append(unit_item!(AmountUnit::Mass(MassUnit::Pound), "pound"))
            .append(unit_item!(AmountUnit::Mass(MassUnit::Gram), "gram"))
            .append(unit_item!(AmountUnit::Mass(MassUnit::Ounce), "ounce"))
            .append(unit_item!(
                AmountUnit::Mass(MassUnit::Milligram),
                "milligram"
            ))
            .append(unit_item!(AmountUnit::Volume(VolumeUnit::Cup), "cup"))
            .append(unit_item!(AmountUnit::Volume(VolumeUnit::Teaspoon), "tsp"))
            .append(unit_item!(
                AmountUnit::Volume(VolumeUnit::Tablespoon),
                "tbsp"
            ))
            .append(unit_item!(
                AmountUnit::Volume(VolumeUnit::FluidOz),
                "fluiz oz"
            ))
            .append(unit_item!(AmountUnit::Volume(VolumeUnit::Pint), "pint"))
            .append(unit_item!(AmountUnit::Volume(VolumeUnit::Liter), "liter"))
            .append(unit_item!(AmountUnit::Volume(VolumeUnit::Quart), "quart"))
            .append(unit_item!(
                AmountUnit::Volume(VolumeUnit::Milliliter),
                "milliliter"
            ));

        if let Event::MouseDown(mouse) = event {
            if ctx.is_hot() {
                ctx.show_context_menu(ContextMenu::new(unit_selector, mouse.window_pos));
            }
        }
        child.event(ctx, event, data, env)
    }
}
