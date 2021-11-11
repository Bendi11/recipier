//! Druid trait implementations for various types

use druid::Data;

use crate::recipes::{db::{Database, RecipeId}, recipe::Recipe};

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