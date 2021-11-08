//! App state used throughout the GUI
use std::{fs::File, path::Path};

use druid::{Data, Lens, widget::ListIter};
use serde::{Deserialize, Serialize};

use crate::recipes::{db::Database, recipe::IngredientAmount};

/// The global app state used in the GUI
#[derive(Debug, Clone, Data, Lens, Deserialize, Serialize)]
pub struct State {
    /// A database of recipes
    pub recipes: Database,
    /// What we are currently rendering on the GUI
    pub screen: AppScreen,
    /// The state of the add screen
    pub add_data: AddState,

    /// The size of the window
    pub window_size: (f64, f64),
}

/// A structure holding the data that can be edited by the user when adding an ingredient
#[derive(Clone, PartialEq, Debug, Data, Lens, Deserialize, Serialize,)]
pub struct EditedIngredient {
    /// The name of the ingredient
    pub name: String,
    /// The amount as a number of the ingredient
    pub amt_num: f32,
    /// The unit to use 
    pub amount: IngredientAmount,
}

/// A newtype over `Vec<EditedIngredients>` used to implement [ListIter](druid::widget::ListIter) for the type
#[derive(Clone, PartialEq, Debug, Data, Deserialize, Serialize,)]
pub struct EditedIngredients(#[data(same_fn="PartialEq::eq")] pub Vec<EditedIngredient>);
impl ListIter<EditedIngredient> for EditedIngredients {
    fn for_each(&self, mut cb: impl FnMut(&EditedIngredient, usize)) {
        for (idx, item) in self.0.iter().enumerate() {
            cb(item, idx)
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut EditedIngredient, usize)) {
        for (idx, item) in self.0.iter_mut().enumerate() {
            cb(item, idx)
        }
    }

    fn data_len(&self) -> usize {
        self.0.len()
    }
}

/// All state in the Add screen
#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize, Lens)]
pub struct AddState {
    /// The name of the added recipe
    name: String,
    /// A list of added ingredients and amounts
    #[data(same_fn="PartialEq::eq")]
    ingredients: Vec<EditedIngredient>,
    /// The instructions text
    instructions: String,
}

/// The screen that the [State] is currently viewing
#[derive(Debug, Clone, Copy, PartialEq, Data, Deserialize, Serialize)]
pub enum AppScreen {
    //View a central list of recipes
    View,
    /// Adding a new recipe
    Add
}

impl State {
    /// Load an app state from a file path or return the default state on error
    pub fn init(path: impl AsRef<Path>) -> Self {
        match File::open(&path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(me) => me,
                Err(e) => {
                    log::error!("Failed to deserialize appstate from file {}: {}, returning default", path.as_ref().display(), e);
                    Self::default()
                }
            },
            Err(e) => {
                log::error!("Failed to open file {}: {}, returning default state", path.as_ref().display(), e);
                Self::default()
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            recipes: Database::new(),
            screen: AppScreen::default(),
            add_data: AddState {
                name: String::new(),
                ingredients: vec![],
                instructions: String::new()
            },
            window_size: (400., 640.)
        }
    }
}
impl Default for AppScreen {
    fn default() -> Self {
        Self::View {

        }
    }
}
impl Data for Database {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}