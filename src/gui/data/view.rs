//! State for the main recipe view screen allowing the user to read a recipe's full contents in one window

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::recipes::db::RecipeId;

/// All data needed by the GUI to render a recipe in the view screen
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct ViewState {
    /// The currently viewed recipe ID
    pub viewed: Option<RecipeId>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            viewed: None
        }
    }
}