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

/// Lens providing access to the search results if they are present
pub struct SearchResultsLens;

impl Lens<SearchState, SearchResults> for SearchResultsLens {
    fn with<V, F: FnOnce(&SearchResults) -> V>(&self, data: &SearchState, f: F) -> V {
        match data.results {
            Some(ref results) => {
                (f)(results)
            },
            None => {
                let mut results = SearchResults {
                    recipes: Vector::new(),
                    term: Arc::from("")
                };
                (f)(&results)
            }
        }
    }

    fn with_mut<V, F: FnOnce(&mut SearchResults) -> V>(&self, data: &mut SearchState, f: F) -> V {
        match data.results {
            Some(ref mut results) => {
                (f)(results)
            },
            None => {
                let mut results = SearchResults {
                    recipes: Vector::new(),
                    term: Arc::from("")
                };
                (f)(&mut results)
            }
        }
    }
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
