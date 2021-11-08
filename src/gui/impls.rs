use std::sync::Arc;

use druid::{Data, widget::ListIter};

use crate::recipes::{db::{Database, RecipeId}, recipe::{Ingredient, IngredientAmount, Recipe}};


impl ListIter<Ingredient> for Arc<[Ingredient]> {
    fn for_each(&self, mut cb: impl FnMut(&Ingredient, usize)) {
        let mut idx = 0;
        for i in self.iter() {
            cb(i, idx);
            idx += 1;
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Ingredient, usize)) {
        if let Some(this) = Arc::get_mut(self) {
            let mut idx = 0;
            for i in this.iter_mut() {
                cb(i, idx);
                idx += 1;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.len()
    }
}

impl druid::Data for Recipe {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}


impl druid::Data for RecipeId {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl druid::widget::ListIter<Recipe> for Database {
    fn for_each(&self, mut cb: impl FnMut(&Recipe, usize)) {
        let mut idx = 0;
        for i in self.iter() {
            cb(&i.1, idx);
            idx += 1;
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Recipe, usize)) {
        let mut idx = 0;
        for mut i in self.iter_mut() {
            cb(&mut i.1, idx);
            idx += 1;
        }
    }

    fn data_len(&self) -> usize {
        self.len()
    }
}

impl Data for IngredientAmount {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}