//! A serializable database containing all recipes

use std::fmt;

use hashbrown::HashMap;
use uuid::Uuid;

use super::recipe::Recipe;

/// A structure holding recipe ID to data pairs with methods to add, remove, and modify recipes
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
struct RecipeEntry {
    /// The recipe data
    recipe: Recipe,

}

/// A unique identifier for a recipe in a database
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecipeId(Uuid);

impl fmt::Display for RecipeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}