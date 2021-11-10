//! Search query and results state

use druid::{Data, Lens, im::Vector};

use crate::recipes::db::RecipeId;

/// Search state containing optional list of search query results and search query details
#[derive(Clone, Debug, Data, Lens, )]
pub struct SearchState {
    /// The search query data
    pub query: Query,
    /// Search results
    pub results: Option<Vector<RecipeId>>,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: Query { 
                term: String::new() 
            },
            results: None
        }
    }
}

/// A search query with all parameters used to find results in a recipe database
#[derive(Clone, Debug, Data, Lens, )]
pub struct Query {
    /// General search term 
    pub term: String,
}
