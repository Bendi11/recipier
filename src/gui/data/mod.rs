pub mod config;
pub mod screen;
pub mod search;
pub mod view;
pub mod home;
pub mod edit;

use std::{ops::Deref, path::Path, sync::Arc};

use druid::{Data, Lens, widget::ListIter};
use serde::{Deserialize, Serialize};

use crate::recipes::{db::Database, recipe::Recipe};

use self::{config::Config, edit::EditState, home::HomeState, screen::AppScreen, search::SearchState, view::ViewState};

/// Structure holding all state information, must be easily cloneable and comparable or performance will
/// suffer
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct AppState {
    /// The configuration data for this state
    pub config: Config,

    /// The central database of recipes
    #[serde(serialize_with = "Database::save", deserialize_with = "Database::load")]
    pub recipes: Database,

    /// State for querying the database of recipes
    #[serde(skip)]
    pub search: SearchState,

    /// The state needed to display the homescreen
    pub home: HomeState,

    /// The screen that the application is currently displaying
    pub screen: AppScreen,

    /// The state for recipe viewing
    pub view: ViewState,

    /// The currently edited recipe state
    pub edit: EditState,

    /// The recipe that the user is being prompted to delete
    pub deleted: Option<Arc<Recipe>>,
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

impl ListIter<Recipe> for AppState {
    fn for_each(&self, mut cb: impl FnMut(&Recipe, usize)) {
        for (i, id) in self.home.loaded.iter().enumerate() {
            match self.recipes.get(*id) {
                Some(recipe) => cb(recipe.deref(), i),
                None => {
                    log::warn!("Loaded recipes contains recipe ID that does not exist");
                }
            }
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Recipe, usize)) {
        for (i, id) in self.home.loaded.iter().enumerate() {
            match self.recipes.get(*id) {
                Some(mut recipe) => {
                    let recipe_ref = Arc::make_mut(&mut recipe);
                    cb(recipe_ref, i);
                    drop(recipe_ref);
                    self.recipes.update(recipe);
                },
                None => {
                    log::warn!("Loaded recipes contains recipe ID that does not exist");
                }
            }
        }
    }

    fn data_len(&self) -> usize {
        self.home.loaded.len()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Config {
                window_size: (480., 700.),
            },
            recipes: Database::new("./recipes"),
            search: SearchState::default(),
            screen: AppScreen::Home,
            home: HomeState::default(),
            view: ViewState::default(),
            edit: EditState::default(),
            deleted: Option::None,
        }
    }
}
