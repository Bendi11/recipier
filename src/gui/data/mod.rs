pub mod config;
pub mod screen;
pub mod search;

use std::path::Path;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::recipes::db::Database;

use self::{config::Config, screen::AppScreen, search::SearchState};

/// Structure holding all state information, must be easily cloneable and comparable or performance will
/// suffer
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct AppState {
    /// The configuration data for this state
    pub config: Config,

    /// The central database of recipes
    #[serde(serialize_with = "Database::save", deserialize_with = "Database::load")]
    pub recipies: Database,

    /// State for querying the database of recipes
    #[serde(skip)]
    pub search: SearchState,

    /// The screen that the application is currently displaying
    pub screen: AppScreen,
}

impl AppState {
    /// Initialize an appstate from a file or the default state if the file was not found
    pub fn init(path: impl AsRef<Path>) -> Self {
        match std::fs::File::open(&path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(me) => me,
                Err(e) => {
                    log::error!(
                        "Failed to deserialize app state from {}: {}, returning default...",
                        path.as_ref().display(),
                        e
                    );
                    Self::default()
                }
            },
            Err(e) => {
                log::warn!(
                    "Failed to open file {}: {} to load stored app state, returning default",
                    path.as_ref().display(),
                    e
                );
                Self::default()
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Config {
                window_size: (480., 700.),
            },
            recipies: Database::new("./recipes"),
            search: SearchState::default(),
            screen: AppScreen::Home,
        }
    }
}
