//! A serializable database containing all recipes

use hashbrown::HashMap;

use super::recipe::Recipe;

/// A structure holding recipe ID to data pairs with methods to add, remove, and modify recipes
#[derive(Clone, Debug)]
pub struct Database {
    /// A map of recipe identifiers to recipe data
    data: HashMap<RecipeId, Recipe>,
}

/// A reference to a recipe in a database
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecipeId(u16);