//! Application state for the remove confirmation prompt

use std::sync::Arc;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::recipes::recipe::Recipe;

use super::screen::AppScreen;

/// All data for the remove prompt, containing what screen to return to after cancel or remove
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct RemoveState {
    /// The recipe that could be deleted
    pub deleted: Arc<Recipe>,
    /// The screen to return to after the prompt is over
    pub return_to: AppScreen,
}