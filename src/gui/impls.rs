//! Druid trait implementations for various types

use druid::Data;

use crate::recipes::db::Database;

impl Data for Database {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}