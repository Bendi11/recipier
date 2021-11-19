//! Widget to display a dropdown used to select an ingredient amount

use druid::{ContextMenu, Event, LocalizedString, MenuDesc, MenuItem, Widget, widget::Controller};

use crate::gui::{CHANGE_INGREDIENT_UNIT, data::{AppState, edit::EditedIngredient}};

/// Widget allowing the user to modify the units of an ingredient amount
pub struct UnitSelectorController;

impl<W: Widget<EditedIngredient>> Controller<EditedIngredient, W> for UnitSelectorController {

    fn event(&mut self, child: &mut W, ctx: &mut druid::EventCtx, event: &Event, data: &mut EditedIngredient, env: &druid::Env) {
        use crate::recipes::measure::*;

        macro_rules! unit_item {
            ($unit:expr, $name:literal) => (MenuItem::new(LocalizedString::new($name), CHANGE_INGREDIENT_UNIT.with((data.id, $unit))) );
        }

        let unit_selector = MenuDesc::<AppState>::new(LocalizedString::new("Unit"))
                .append(unit_item!(AmountUnit::Mass(MassUnit::Pound), "pound"));

        
        if let Event::MouseDown(mouse) = event {
            if ctx.is_hot() {
                ctx.show_context_menu(ContextMenu::new(unit_selector, mouse.window_pos));
            }
        }
        child.event(ctx, event, data, env)
    }
}