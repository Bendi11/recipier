//! Druid trait implementations for various types

use std::sync::Arc;

use druid::{Data, widget::ListIter};

use crate::recipes::{db::{Database, RecipeId}, recipe::Recipe};

use super::data::search::SearchResults;

impl Data for Database {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Data for RecipeId {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Data for Recipe {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
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