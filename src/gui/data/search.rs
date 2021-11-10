//! Search query and results state

use std::sync::Arc;

use druid::{Data, Lens, im::Vector};

use crate::recipes::db::RecipeId;

/// Search state containing optional list of search query results and search query details
#[derive(Clone, Debug, Data, Lens, )]
pub struct SearchState {
    /// The search query data
    pub query: Query,
    /// Search results
    pub results: Option<SearchResults>,
}

/// The result of searching for a query
#[derive(Clone, Debug, Data, Lens, )]
pub struct SearchResults {
    /// List of recipes matching the query
    pub recipes: Vector<RecipeId>,
    /// The original search term 
    pub term: Arc<str>,
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
