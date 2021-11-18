//! State for the home screen like number of displayed recipes

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

/// Structure holding all state needed in the home screen widget
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct HomeState {
    /// How many recipes are currently loaded in view
    #[serde(skip, default = "default_loaded")]
    pub loaded: u32,
}

/// Function to provide a default value to the `loaded` count of the [HomeState] struct when deserializing with serde
pub fn default_loaded() -> u32 {
    10
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            loaded: default_loaded()
        }
    }
}