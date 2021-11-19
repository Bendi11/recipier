//! State for the currently edited recipe

use std::{ops::Deref, time::Duration};

use druid::{im::Vector, Data, Lens};
use serde::{Deserialize, Serialize};

use crate::recipes::{
    db::RecipeId,
    recipe::{Ingredient, IngredientAmount, Recipe},
};

use super::screen::AppScreen;

/// Data for the currently edited recipe
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct EditState {
    /// If we are modifying the recipe, this is the ID of the modified recipe
    pub id: Option<RecipeId>,
    /// The name of the recipe
    pub title: String,
    /// Ingredients list
    pub ingredients: Vector<EditedIngredient>,
    /// Body of the recipe
    pub body: String,
    /// Number of servings the recipe makes
    pub servings: Option<f32>,
    /// The amount of time that the recipe is expected to take
    pub time: Option<EditedTime>,
    /// The screen to return to after editing is over
    pub return_to: AppScreen,
}

/// Data for a user-edited time that destructures a [Duration](std::time::Duration)
#[derive(Clone, Copy, Debug, Default, Data, Lens, Serialize, Deserialize)]
pub struct EditedTime {
    /// Seconds component of the time
    pub secs: u8,
    /// Minutes of the time
    pub minutes: u8,
    /// Hours of the time
    pub hours: u8,
}

impl From<Duration> for EditedTime {
    fn from(duration: Duration) -> Self {
        let time = duration.as_secs_f32();
        let hours = time / 360f32;
        let minutes = (time - (hours.trunc() * 360f32)) / 60.;
        let seconds = time - ((hours.trunc() * 360f32) - (hours.trunc() * 60f32));
        log::trace!(
            "Converted {}s to {} hours, {} mins, {} secs",
            time,
            hours,
            minutes,
            seconds
        );
        Self {
            secs: seconds as u8,
            minutes: minutes as u8,
            hours: hours as u8,
        }
    }
}

/// Ingredient data stored in a more efficiently mutable way
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct EditedIngredient {
    /// The name of the ingredient
    pub name: String,
    /// The amount of the ingredient needed
    #[data(same_fn = "PartialEq::eq")]
    pub amount: IngredientAmount,
}

impl From<&Ingredient> for EditedIngredient {
    fn from(ingredient: &Ingredient) -> Self {
        Self {
            name: ingredient.name.deref().to_owned(),
            amount: ingredient.amount,
        }
    }
}

impl From<&Recipe> for EditState {
    fn from(recipe: &Recipe) -> Self {
        Self {
            id: Some(recipe.id),
            title: recipe.name.deref().to_owned(),
            ingredients: recipe
                .ingredients
                .iter()
                .map(EditedIngredient::from)
                .collect(),
            body: recipe.body.deref().to_owned(),
            servings: recipe.servings,
            time: recipe.time.map(From::from),
            return_to: AppScreen::Home,
        }
    }
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
            ingredients: Vector::new(),
            body: String::new(),
            servings: None,
            time: None,
            return_to: AppScreen::Home,
        }
    }
}
