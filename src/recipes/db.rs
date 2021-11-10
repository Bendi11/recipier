//! A serializable database containing all recipes

use std::{fmt, fs::File, path::Path, sync::Arc};

use hashbrown::HashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use super::recipe::Recipe;

/// A structure holding recipe ID to data pairs with methods to add, remove, and modify recipes
/// 
/// The structure is an Arc over internal state, so is very easy to clone
#[derive(Clone, Debug)]
pub struct Database {
    /// A map of recipe IDs to loaded or unloaded recipe data
    items: Arc<RwLock<HashMap<RecipeId, DbEntry>>>,
    /// The directory that all recipe files are stored in
    dir: Arc<Path>,
}

/// An enum representing the stage that a recipe entry is in loading
#[derive(Clone, Debug)]
enum DbEntry {
    /// There is no recipe present
    Unloaded,
    /// The recipe has been loaded
    Loaded(Arc<Recipe>),
}

impl Database {
    /// Get a recipe by UUID from this database, if the recipe is not currently loaded then it will be loaded
    pub fn get(&self, id: RecipeId) -> Option<Arc<Recipe>> {
        let items = self.items.read();
        match items.get(&id)? {
            DbEntry::Unloaded => {
                drop(items);
                let mut items = self.items.write();
                match File::open(self.dir.join(id.to_string())) {
                    Ok(file) => match serde_json::from_reader(file) {
                        Ok(recipe) => {
                            log::trace!("Loaded recipe {} from file after get requested", id);
                            
                            let recipe: Arc<Recipe> = Arc::new(recipe);
                            items.insert(id, DbEntry::Loaded(recipe.clone()));
                            Some(recipe)
                        },
                        Err(e) => {
                            log::error!("Failed to deserialize recipe from file {}: {}", id, e);
                            None
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to load recipe from file {}: {}", id, e);
                        None
                    }
                }
            },
            DbEntry::Loaded(recipe) => Some(recipe.clone())
        }
    }

    /// Get the number of recipes in this database
    pub fn len(&self) -> usize {
        self.items.read().len()
    }

    /// Create a new empty database
    pub fn new(path: impl AsRef<Path>) -> Self {
        let this = Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            dir: Arc::from(path.as_ref())
        };
        //this.insert(Recipe::top_ramen());
        this
    }

    /// Insert a recipe into the database, automatically creating an ID and returning it
    pub fn insert(&self, recipe: Recipe) -> RecipeId {
        let mut data = self.items.write();
        loop {
            let id = RecipeId(Uuid::new_v4());
            match data.contains_key(&id) {
                true => {
                    log::warn!(
                        "Database already contains recipe with ID {}, re-generating...",
                        id
                    );
                    continue;
                }
                false => {
                    data.insert(id, DbEntry::Loaded(Arc::new(recipe)));
                    log::trace!("inserting recipe with ID {} into database...", id);
                }
            }
            break id;
        }
    }

    /// Save this database to a directory of files and a path to the directory
    pub fn save<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if let Err(e) = std::fs::create_dir_all(&self.dir) {
            log::error!("Failed to create directory {} to save recipes to: {}", self.dir.display(), e);
            return self.dir.serialize(ser)
        }

        let items = self.items.read();
        for (id, recipe) in items.iter() {
            if let DbEntry::Loaded(recipe) = recipe {
                let path = self.dir.join(id.to_string());
                match File::create(&path) {
                    Ok(file) => if let Err(e) = serde_json::to_writer_pretty(file, recipe) {
                        log::error!("Failed to serialize recipe {} to {}: {}", id, path.display(), e);
                    },
                    Err(e) => {
                        log::error!("Failed to create / overwrite file {}: {}", path.display(), e);
                    }
                }
            }
        }

        self.dir.serialize(ser)
    }

    /// Load this database from a path item and directory with recipe data
    pub fn load<'de, D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let dir_path = Arc::<Path>::deserialize(de)?; //Load the directory path
        let this = Self::new(&dir_path);

        match std::fs::read_dir(&dir_path) {
            Ok(dir) => {
                for item in dir {
                    match item {
                        Ok(item) => if let Some(name) = item.file_name().to_str() {
                            match Uuid::parse_str(name) {
                                Ok(id) => {
                                    log::trace!("Adding recipe file {} to map as unloaded...", id);
                                    this.items.write().insert(RecipeId(id), DbEntry::Unloaded);
                                },
                                Err(e) => {
                                    log::warn!("Failed to parse directory item {} as UUID, not adding as entry...", e);
                                }
                            }
                        },
                        Err(e) => {
                            log::warn!("Failed to get directory entry in {}: {}", dir_path.display(), e);
                        }
                    }
                }
            },
            Err(e) => {
                log::error!("Failed to open recipe directory {}: {}", dir_path.display(), e);
                return Ok(this)
            }
        }

        Ok(this)
    }
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.items, &other.items)
    }
}

/// A unique identifier for a recipe in a database
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RecipeId(Uuid);

impl fmt::Display for RecipeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
