//! A serializable database containing all recipes

use std::fmt;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::recipe::Recipe;

/// A structure holding recipe ID to data pairs with methods to add, remove, and modify recipes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    /// A map of recipe IDs to recipe data
    data: HashMap<RecipeId, RecipeEntry>,
}

impl Database {
    /// Get a recipe by UUID from this database
    #[inline]
    pub fn get(&self, id: RecipeId) -> Option<&Recipe> {
        self.data.get(&id).map(|entry| &entry.recipe)
    }

    /// Create a new empty database
    pub fn new() -> Self {
        let mut this = Self {
            data: HashMap::new()
        };
        this.insert(Recipe::top_ramen());
        this
    }

    /// Insert a recipe into the database, automatically creating an ID and returning it
    pub fn insert(&mut self, recipe: Recipe) -> RecipeId {
        loop {
            let id = RecipeId(Uuid::new_v4());
            match self.data.contains_key(&id) {
                true => {
                    log::warn!("Database already contains recipe with ID {}, re-generating...", id);
                    continue
                },
                false => {
                    self.data.insert(id, RecipeEntry {
                        recipe
                    });
                    log::trace!("inserting recipe with ID {} into database...", id);
                }
            }
            break id
        }
    }
}


/// Structure used as values in the [Database], containing recipe data + metadata only used
/// internally in the database
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct RecipeEntry {
    /// The recipe data
    recipe: Recipe,

}

/// A unique identifier for a recipe in a database
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,)]
pub struct RecipeId(Uuid);

impl druid::Data for RecipeId {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl druid::widget::ListIter<Recipe> for Database {
    fn for_each(&self, mut cb: impl FnMut(&Recipe, usize)) {
        let mut idx = 0;
        for i in self.data.iter() {
            cb(&i.1.recipe, idx);
            idx += 1;
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Recipe, usize)) {
        let mut idx = 0;
        for i in self.data.iter_mut() {
            cb(&mut i.1.recipe, idx);
            idx += 1;
        }
    }

    fn data_len(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for RecipeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}