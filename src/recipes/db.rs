//! A serializable database containing all recipes

use std::{borrow::Borrow, fmt, fs::File, ops::Deref, path::Path, sync::Arc};

use druid::im::OrdMap;
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
    items: Arc<RwLock<HashMap<RecipeId, Arc<Recipe>>>>,
    /// The directory that all recipe files are stored in
    dir: Arc<Path>,
}

impl Database {
    /// Get a recipe by UUID from this database, if the recipe is not currently loaded then it will be loaded
    pub fn get(&self, id: RecipeId) -> Option<Arc<Recipe>> {
        let items = self.items.read();
        items.get(&id).map(|recipe| recipe.clone())
    }

    /// Search this database, returning an ordered map of scores to recipe data
    pub fn search(&self, searcher: impl Fn(&Recipe) -> isize) -> OrdMap<isize, Arc<Recipe>> {
        let mut results = OrdMap::new();
        let items = self.items.read();

        for (_, recipe) in items.iter() {
            results.insert(searcher(recipe.borrow()), recipe.clone());
        }

        results
    }

    /// Update a recipe with new data
    pub fn update(&self, recipe: Arc<Recipe>) {
        let mut items = self.items.write();
        match items.get_mut(&recipe.id) {
            Some(entry) => {
                *entry = recipe;
            }
            None => (),
        }
    }

    /// Get the number of recipes in this database
    pub fn len(&self) -> usize {
        self.items.read().len()
    }

    /// Create a new empty database
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            dir: Arc::from(path.as_ref()),
        }
    }

    /// Insert a recipe into the database, automatically creating an ID and returning it
    pub fn insert(&self, recipe: Recipe) {
        let mut data = self.items.write();
        match data.contains_key(&recipe.id) {
            true => {
                log::trace!(
                    "Database already contains recipe with ID {}, updating instead...",
                    recipe.id
                );
                data.insert(recipe.id, Arc::new(recipe));
            }
            false => {
                log::trace!("inserting recipe with ID {} into database...", recipe.id);
                data.insert(recipe.id, Arc::new(recipe));
            }
        }
    }

    /// Save this database to a directory of files and a path to the directory
    pub fn save<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        if let Err(e) = std::fs::create_dir_all(&self.dir) {
            log::error!(
                "Failed to create directory {} to save recipes to: {}",
                self.dir.display(),
                e
            );
            return self.dir.serialize(ser);
        }

        let items = self.items.read();
        for (id, recipe) in items.iter() {
            let path = self.dir.join(id.to_string());
            match File::create(&path) {
                Ok(file) => {
                    if let Err(e) = serde_json::to_writer_pretty(file, recipe) {
                        log::error!(
                            "Failed to serialize recipe {} to {}: {}",
                            id,
                            path.display(),
                            e
                        );
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to create / overwrite file {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }

        self.dir.serialize(ser)
    }

    /// Load this database from a path item and directory with recipe data
    pub fn load<'de, D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let dir_path = Arc::<Path>::deserialize(de)?; //Load the directory path
        let this = Self::new(&dir_path);
        //this.insert(Recipe::top_ramen());

        match std::fs::read_dir(&dir_path) {
            Ok(dir) => {
                for item in dir {
                    match item {
                        Ok(item) => {
                            if let Some(name) = item.file_name().to_str() {
                                match Uuid::parse_str(name) {
                                    Ok(id) => {
                                        log::trace!("Adding recipe file {} to db...", id);

                                        match File::open(item.path()) {
                                            Ok(file) => match serde_json::from_reader(file) {
                                                Ok(recipe) => {
                                                    let recipe: Arc<Recipe> = Arc::new(recipe);
                                                    this.items.write().insert(RecipeId(id), recipe);
                                                }
                                                Err(e) => {
                                                    log::error!("Failed to deserialize recipe from file {}: {}", id, e);
                                                }
                                            },
                                            Err(e) => {
                                                log::error!(
                                                    "Failed to load recipe from file {}: {}",
                                                    id,
                                                    e
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::warn!("Failed to parse directory item {} as UUID, not adding as entry...", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!(
                                "Failed to get directory entry in {}: {}",
                                dir_path.display(),
                                e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "Failed to open recipe directory {}: {}",
                    dir_path.display(),
                    e
                );
                return Ok(this);
            }
        }

        Ok(this)
    }

    /// Remove the recipe with the specified ID from the recipes database
    pub fn remove(&self, id: RecipeId) {
        self.items.write().remove(&id);
        let file_path = self.dir.join(id.to_string());
        //Remove the recipe file as well
        if let Err(e) = std::fs::remove_file(self.dir.join(&file_path)) {
            log::warn!(
                "Failed to remove recipe {} save file {}: {}",
                id,
                file_path.display(),
                e
            );
        } else {
            log::trace!("Removed save file and db entry for recipe {}", id);
        }
    }

    /// Get an iterator over all ids for recipes in this database
    pub fn ids(&self) -> Arc<[RecipeId]> {
        let items = self.items.read();
        items.iter().map(|(id, _)| *id).collect()
    }
}

impl druid::widget::ListIter<Recipe> for Database {
    fn for_each(&self, mut cb: impl FnMut(&Recipe, usize)) {
        let items = self.items.read();
        for (i, (_, recipe)) in items.iter().enumerate() {
            cb(recipe.deref(), i)
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Recipe, usize)) {
        let mut items = self.items.write();
        for (i, (_, recipe)) in items.iter_mut().enumerate() {
            let recipe = Arc::make_mut(recipe);
            cb(recipe, i)
        }
    }

    fn data_len(&self) -> usize {
        self.items.read().len()
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

impl RecipeId {
    /// Create a new unique recipe ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for RecipeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
