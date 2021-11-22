//! State for the currently edited recipe

use std::{ops::Deref, sync::Arc, time::Duration};

use druid::{Data, ImageBuf, Lens, im::HashMap, widget::ListIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::recipes::{db::{Database, RecipeId}, measure::{AmountUnit, Mass, Volume}, recipe::{Ingredient, IngredientAmount, Recipe}};

use super::screen::AppScreen;

/// Data for the currently edited recipe
#[derive(Clone, Debug, Lens, Serialize, Deserialize)]
pub struct EditState {
    /// If we are modifying the recipe, this is the ID of the modified recipe
    pub id: Option<RecipeId>,
    /// The name of the recipe
    pub title: String,
    /// Ingredients list, hashmap to make removal of certain items more eficient
    pub ingredients: HashMap<Uuid, EditedIngredient>,
    /// Body of the recipe
    pub body: Arc<String>,
    /// Number of servings the recipe makes
    pub servings: Option<f32>,
    /// The amount of time that the recipe is expected to take
    pub time: Option<EditedTime>,
    /// Optional image data
    #[serde(skip)]
    pub image: Option<ImageBuf>,
    /// The screen to return to after editing is over
    pub return_to: AppScreen,
}

impl Data for EditState {
    fn same(&self, other: &Self) -> bool {
        self.id.same(&other.id) &&
        self.title.same(&other.title) &&
        self.ingredients.same(&other.ingredients) &&
        self.body.same(&other.body) &&
        self.servings.same(&other.servings) &&
        self.time.same(&other.time) && 
        match (&self.image, &other.image) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(ref img1), Some(ref img2)) => Arc::ptr_eq(&img1.raw_pixels_shared(), &img2.raw_pixels_shared())
        } &&
        self.return_to.same(&other.return_to)
        
    }
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
        let hours = time / 3600f32;
        let minutes = (time - (hours.trunc() * 3600f32)) / 60.;
        let seconds = time - (hours.trunc() * 3600f32) - (minutes.trunc() * 60f32);

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
    /// The id that this ingredient is associated with in the edit state
    #[data(same_fn = "PartialEq::eq")]
    pub id: Uuid,
    /// The name of the ingredient
    pub name: Arc<String>,
    /// The number of the given unit for the ingredient
    pub count: f32,
    /// The amount of the ingredient needed
    pub unit: AmountUnit,
    /// Opposite of if the ingredient is optional
    pub required: bool,
}

impl EditedIngredient {
    /// Create a new edited ingredient with the given ID
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            name: Arc::from("".to_owned()),
            count: 0.,
            unit: AmountUnit::None,
            required: true,
        }
    }

    /// Create a new immutable ingredient from this edited ingredient
    pub fn to_ingredient(&self) -> Ingredient {
        Ingredient {
            name: Arc::from(self.name.deref().as_str()),
            amount: match self.unit {
                AmountUnit::None => IngredientAmount::None,
                AmountUnit::Count => IngredientAmount::Count(self.count),
                AmountUnit::Mass(unit) => IngredientAmount::Mass(Mass {
                    val: self.count,
                    unit,
                }),
                AmountUnit::Volume(unit) => IngredientAmount::Volume(Volume {
                    val: self.count,
                    unit,
                }),
            },
            optional: !self.required,
        }
    }

    /// Create a new edited ingredient using data from an existing ingredient, used for editing
    fn from_ingredient(id: Uuid, ingredient: &Ingredient) -> Self {
        Self {
            id,
            count: match ingredient.amount {
                IngredientAmount::Count(n) => n as f32,
                IngredientAmount::Mass(m) => m.val,
                IngredientAmount::Volume(v) => v.val,
                IngredientAmount::None => 0f32,
            },
            name: Arc::new(ingredient.name.deref().to_owned()),
            unit: ingredient.amount.into(),
            required: !ingredient.optional,
        }
    }
}

impl EditState {
    pub fn from_recipe(db: &Database, recipe: &Recipe) -> Self {
        Self {
            id: Some(recipe.id),
            title: recipe.name.deref().to_owned(),
            ingredients: recipe
                .ingredients
                .iter()
                .map(|v| {
                    let id = Uuid::new_v4();
                    (id, EditedIngredient::from_ingredient(id, v))
                })
                .collect(),
            body: Arc::new(recipe.body.deref().to_owned()),
            servings: recipe.servings,
            time: recipe.time.map(From::from),
            return_to: AppScreen::Home,
            image: db.get_image(recipe.id),
        }
    }
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            id: None,
            title: String::new(),
            ingredients: HashMap::new(),
            body: Arc::new(String::new()),
            servings: None,
            time: None,
            return_to: AppScreen::Home,
            image: None,
        }
    }
}

impl ListIter<EditedIngredient> for EditState {
    fn for_each(&self, mut cb: impl FnMut(&EditedIngredient, usize)) {
        for (i, (_, val)) in self.ingredients.iter().enumerate() {
            cb(val, i)
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut EditedIngredient, usize)) {
        for (i, (_, val)) in self.ingredients.iter_mut().enumerate() {
            cb(val, i)
        }
    }

    fn data_len(&self) -> usize {
        self.ingredients.len()
    }
}
