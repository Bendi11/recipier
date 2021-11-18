//! Druid trait implementations for various types

use druid::{widget::ListIter, Data};

use crate::recipes::{
    db::{Database, RecipeId},
    recipe::{Ingredient, Recipe},
};

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


impl ListIter<Ingredient> for Recipe {
    fn for_each(&self, mut cb: impl FnMut(&Ingredient, usize)) {
        for (i, ingredient) in self.ingredients.iter().enumerate() {
            cb(ingredient, i)
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Ingredient, usize)) {
        for (i, ingredient) in self.ingredients.iter_mut().enumerate() {
            cb(ingredient, i)
        }
    }

    fn data_len(&self) -> usize {
        self.ingredients.len()
    }
}
