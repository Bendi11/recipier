//! State for the home screen like number of displayed recipes

use druid::{Data, Lens, im::Vector};
use serde::{Deserialize, Serialize};

use crate::recipes::db::RecipeId;

/// Structure holding all state needed in the home screen widget
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct HomeState {
    /// How many recipes are currently loaded in view
    #[serde(skip)]
    pub loaded: Vector<RecipeId>
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            loaded: Vector::default()
        }
    }
}