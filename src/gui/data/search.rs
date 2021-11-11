//! Search query and results state

use std::sync::Arc;

use druid::{Data, Lens, im::OrdMap, widget::ListIter};

use crate::recipes::recipe::Recipe;

/// Search state containing optional list of search query results and search query details
#[derive(Clone, Debug, Data, Lens)]
pub struct SearchState {
    /// The search query data
    pub query: Query,
    /// Search results
    pub results: Option<SearchResults>,
}

/// The result of searching for a query
#[derive(Clone, Debug, Data, Lens)]
pub struct SearchResults {
    /// List of recipes matching the query
    pub recipes: OrdMap<isize, Arc<Recipe>>,
    /// The number of recipes that should be loaded onscreen
    pub loaded_recipes: usize,
    /// The original search term
    pub term: Arc<str>,
}

impl ListIter<Arc<Recipe>> for SearchResults {
    fn for_each(&self, mut cb: impl FnMut(&Arc<Recipe>, usize)) {
        for (idx, (_, i)) in self.recipes.iter().take(self.loaded_recipes).enumerate() {
            (cb)(i, idx)
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Arc<Recipe>, usize)) {
        let mut recipes = self.recipes.clone();
        for (idx, (score, i)) in self.recipes.iter().take(self.loaded_recipes).enumerate() {
            let mut recipe = i.clone();
            (cb)(&mut recipe, idx);
            if !Arc::ptr_eq(&recipe, i) {
                recipes = self.recipes.alter(|_| Some(recipe), *score);
            }
        }
        self.recipes = recipes;
    }

    fn data_len(&self) -> usize {
        self.recipes.len().min(self.loaded_recipes)
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: Query {
                term: String::new(),
            },
            results: None,
        }
    }
}

/// A search query with all parameters used to find results in a recipe database
#[derive(Clone, Debug, Data, Lens)]
pub struct Query {
    /// General search term
    pub term: String,
}
