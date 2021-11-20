pub mod data;
pub mod handler;
mod impls;
pub mod theme;
pub mod ui;
pub mod widgets;

use crate::recipes::{db::RecipeId, measure::AmountUnit};

use self::data::screen::AppScreen;
use druid::Selector;
pub use ui::root_widget;
use uuid::Uuid;

/// The visually-appealing golden ratio
pub const GOLDEN_RATIO: f64 = 1.61803;

/// Change the currently visisble screen command
pub const CHANGE_SCREEN: Selector<AppScreen> = Selector::new("recipier.change-screen");

/// Populate search results with data from the search query state
pub const POPULATE_RESULTS: Selector = Selector::new("recipier.populate-search-results");

/// View the specified recipe in the recipe view screen
pub const VIEW_RECIPE: Selector<RecipeId> = Selector::new("recipier.view-recipe");

/// Load more recipes into the recipes home screen
pub const LOAD_MORE_RECIPES: Selector = Selector::new("recipeier.load-more-recipes");

/// Copy the given recipe by ID to the edit state data and return to the given screen when editing is finished
pub const EDIT_RECIPE: Selector<(RecipeId, AppScreen)> = Selector::new("recipier.edit-recipe");

/// Wipe existing edit data to start with a blank state
pub const CREATE_RECIPE: Selector = Selector::new("recipier.create-recipe");

/// Remove the recipe with the given ID and return to the given screen after displaying the
/// delete prompt
pub const REMOVE_RECIPE: Selector<(RecipeId, AppScreen)> = Selector::new("recipier.remove-recipe");

/// Change the selected ingredient's unit to the given unit
pub const CHANGE_INGREDIENT_UNIT: Selector<(Uuid, AmountUnit)> =
    Selector::new("recipier.change-ingredient-unit");

/// Remove the ingredient with the specified ID from the currently edited ingredients
pub const REMOVE_EDITED_INGREDIENT: Selector<Uuid> = Selector::new("recipier.remove-edited-ingredient");
