use eframe::{
    egui::{self, Key, TextEdit, Visuals, Widget, Label, Button, Color32}, 
    epi::App
};
use serde::{Serialize, Deserialize};
use std::{collections::BTreeMap, fmt, time};
use generational_arena::{Arena, Index};
use crate::{measure::{Mass, MassUnit, TimeUnit, Volume, VolumeUnit}, recipe::{Ingredient, IngredientAmount, Recipe}};

const SAVE_FILE: &str = "./recipes.json";


/// Recipe application holding all state for the GUI and all recipes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecipeApp {
    /// An allocator for recipes, so that recipes can be addressed by handle instead of
    /// needing smart pointers or references
    pub recipes: Arena<Recipe>,

    /// What screen is being displayed
    view: View,

    /// A list of recipes that we have searched for
    search: SearchState,

    /// Edit state
    edit: EditState,
}


impl RecipeApp {
    /// Attempt to load a saved recipe file, or create a default recipe app with no recipes
    pub fn load_or_default() -> Self {
        if let Ok(file) = std::fs::File::open(SAVE_FILE) {
            if let Ok(app) = serde_json::from_reader(file) {
                return app
            }
        }

        Self {
            recipes: Arena::new(),
            view: View::Overview,
            edit: EditState::default(),
            search: SearchState::default(),            
        }
    }

    /// Show the top menu bar, returning `true` if a button was pressed
    fn top_menubar(&mut self, ctx: &egui::CtxRef) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.add_space(2.);
            ui.columns(2, |ui| {

                if ui[1].add(Button::new("Home")).clicked() {
                    self.view = View::Overview;
                } else if ui[0].add(Button::new("Add")).clicked() {
                    self.edit.reset();
                    let new = self.recipes.insert(Recipe::default());
                    self.edit.editing_recipe = Some(new);
                    self.view = View::EditRecipe;
                } 
            });


            let search = TextEdit::singleline(&mut self.search.term)
                .hint_text("search");
            ui.add(search);
            if ctx.input().key_pressed(Key::Enter) {
                self.search.matched = {
                    let mut matches = BTreeMap::new();
                    for (idx, recipe) in self.recipes.iter() {
                        let mut scores = [isize::MIN, isize::MIN, isize::MIN];
                        if self.search.in_titles {
                            if let Some(score) = sublime_fuzzy::best_match(self.search.term.as_str(), recipe.name.as_str()) {
                                scores[0] = score.score();
                            }
                        }
                        if self.search.in_body {
                            if let Some(score) = sublime_fuzzy::best_match(self.search.term.as_str(), recipe.body.as_str()) {
                                scores[1] = score.score();
                            }
                        }
                        if self.search.in_ingredients {
                            if let Some(score) = recipe.ingredients
                                .iter()
                                .map(|i| sublime_fuzzy::best_match(self.search.term.as_str(), i.name.as_str()).map(|s| s.score()).unwrap_or(isize::MIN))
                                .max() {
                                scores[2] = score;
                            }
                        }
                        
                        let max = scores.iter().max().unwrap();
                        if max >= &self.search.threshold {
                            matches.insert(*max, idx);
                        }
                    }
                    matches
                };
                self.view = View::Search;
            }

            
            ui.collapsing("Advanced Search", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.search.in_titles, "Search in titles");
                    ui.checkbox(&mut self.search.in_body, "Search in recipe body");
                    ui.checkbox(&mut self.search.in_ingredients, "Search in recipe ingredients");
                    egui::Slider::new(&mut self.search.threshold, 0isize..=100)
                        .text("Search threshold")
                        .ui(ui)
                        .on_hover_text("Adjust how little a recipe must match the search term to be included in results");
                })
            });
        });
    }

    /// Show a widget for a recipe
    fn show_recipe(ui: &mut egui::Ui, idx: Index, recipes: &Arena<Recipe>, view: &mut View, edit: &mut EditState) {
        if let Some(recipe) = recipes.get(idx) {
            ui.columns(3, |ui| {
                ui[0].heading(&recipe.name);
                ui[0].add_space(10.);
    
                if ui[1].button("Edit").clicked() {
                    *view = View::EditRecipe;
                    *edit = EditState::from_recipe(recipe, idx);
                }
                if ui[2].button("Delete").clicked() {
                    *view = View::AreYouSure;
                    *edit = EditState::from_recipe(recipe, idx);
                }
            });
    
            egui::CollapsingHeader::new("Ingredients")
                .id_source(idx)
                .show(ui, |ui| {
                    for ingredient in recipe.ingredients.iter() {
                        ui.label(format!("- {}", ingredient.to_string()));
                    }
                });
    
            ui.separator();
    
            if let Some(time) = recipe.time {
                ui.label(format!("Time: {}", humantime::format_duration(time)));
            }
            let label = Label::new(&recipe.body).wrap(true);
            ui.group(|ui| label.ui(ui));
        }
    }
}

impl App for RecipeApp {
    fn name(&self) -> &str {
        "Recipier"
    }

    fn setup(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>, _storage: Option<&dyn eframe::epi::Storage>) {
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals::light();
        ctx.set_style(style);
        
    }

    fn on_exit(&mut self) {
        if let Ok(save) = std::fs::File::create(SAVE_FILE) {
            let _ = serde_json::to_writer(save, &self);
        }
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.view {
            View::Overview => {
                self.top_menubar(ctx);

                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.recipes.is_empty() {
                        ui.add(Label::new("No Recipes Yet!"));
                    } else {
                        for (idx, _) in self.recipes.iter() {
                            Self::show_recipe(ui, idx, &self.recipes, &mut self.view, &mut self.edit);
                            ui.separator();
                        }
                    }
                });
            },  
            View::Search => {
                self.top_menubar(ctx);

                egui::CentralPanel::default().show(ctx, |ui| {
                    for (_, idx) in self.search.matched.iter() {
                        let idx = *idx;
                        Self::show_recipe(ui, idx, &self.recipes, &mut self.view, &mut self.edit);
                        ui.separator();
                    }
                });

            },
            View::EditRecipe => {
                egui::TopBottomPanel::top("menu_bar_top").show(ctx, |ui| {
                    ui.columns(3, |ui| {
                        if ui[0].button("Cancel")
                            .on_hover_text("Return to the hompage without editing")
                            .clicked() {
                            self.edit.reset();
                            self.view = View::Overview;
                        }
                        if ui[1].button("Delete")
                            .on_hover_text("Delete the edited recipe")
                            .clicked() {
                            if self.edit.editing_recipe.is_some() {
                                self.view = View::AreYouSure;
                            }
                        }
                        if ui[2].button("Save and Exit")
                            .on_hover_text("Save edits to the current recipe")
                            .clicked() {
                            self.edit.wrongs.clear();
                            match self.edit.to_recipe() {
                                Ok(new) => {
                                    if let Some(idx) = self.edit.editing_recipe {
                                        *self.recipes.get_mut(idx).unwrap() = new;
                                        self.view = View::Overview;
                                    }
                                },
                                Err(e) => self.edit.wrongs.push(e)
                            }

                        }
                    })
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    for wrong in self.edit.wrongs.iter() {
                        ui.colored_label(Color32::from_rgb(255, 61, 61), wrong.to_string());
                    }
                    ui.small("Title");
                    ui.text_edit_singleline(&mut self.edit.name).on_hover_text("Edit the title of the recipe");
                    ui.separator();

                    match self.edit.adding_time {
                        true => {
                            ui.small("Time");
                            ui.text_edit_singleline(&mut self.edit.time_string);
                            egui::ComboBox::from_label("unit")
                                .selected_text(self.edit.time_unit.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Second, "second");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Minute, "minute");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Hour, "hour");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Day, "day");
                                });
                            if ui.small_button("Remove Time").clicked() {
                                self.edit.adding_time = false;
                            }
                        },
                        false => if ui.button("Add Time").on_hover_text("Add an estimate for how long the recipe takes to make").clicked() {
                            self.edit.adding_time = true;
                        }
                    }

                    ui.separator();
                    ui.collapsing("Ingredients", |ui| {
                        let mut remove = None;
                        for (idx, (name, amount)) in self.edit.ingredients.iter_mut().enumerate() {
                            ui.columns(4, |ui| {
                                ui[0].text_edit_singleline(name).on_hover_text("Ingredient Name");
                                match amount {
                                    Some((amount, unit)) => {
                                        ui[1].text_edit_singleline(amount).on_hover_text("Ingredient Amount");
                                        egui::ComboBox::from_id_source(idx)
                                            .selected_text(unit.unit_string())
                                            .show_ui(&mut ui[2], |ui| {
                                                ui.selectable_value(unit, IngredientAmount::None, "no unit");

                                                ui.selectable_value(unit, IngredientAmount::Mass(Mass { val: 0., unit: MassUnit::Ounce}), "ounce");
                                                ui.selectable_value(unit, IngredientAmount::Mass(Mass { val: 0., unit: MassUnit::Pound}), "pound");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Cup}), "cup");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Tablespoon}), "tablespoon");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Teaspoon}), "teaspoon");

                                                ui.selectable_value(unit, IngredientAmount::Count(0), "count");

                                                ui.selectable_value(unit, IngredientAmount::Mass(Mass { val: 0., unit: MassUnit::Gram}), "gram");
                                                ui.selectable_value(unit, IngredientAmount::Mass(Mass { val: 0., unit: MassUnit::Kilogram}), "kilogram");
                                                ui.selectable_value(unit, IngredientAmount::Mass(Mass { val: 0., unit: MassUnit::Milligram}), "milligram");
                                            
                                                
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::FluidOz}), "fluid ounce");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Liter}), "liter");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Milliliter}), "milliliter");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Pint}), "pint");
                                                ui.selectable_value(unit, IngredientAmount::Volume(Volume { val: 0., unit: VolumeUnit::Quart}), "quart");
                                                
                                            });
                                    },
                                    None => if ui[1].small_button("Add Amount").clicked() {
                                        *amount = Some((String::new(), IngredientAmount::None));
                                    }
                                }
                                
                                if ui[3].button("Remove").clicked() {
                                    remove = Some(idx);
                                }
                            });
                        }
                        if let Some(idx) = remove {
                            self.edit.ingredients.remove(idx);
                        }
                        
                        if ui.button("+ Add Ingredient").clicked() {
                            self.edit.ingredients.push((String::new(), None));
                        }
                    });

                    ui.separator();
                    ui.label("Recipe Body");
                    ui.add(egui::TextEdit::multiline(&mut self.edit.body)
                        .desired_width(ui.available_width())
                    );
                    
                });
            },
            View::AreYouSure => {
                if let Some(idx) = self.edit.editing_recipe.as_ref() {
                    let idx = *idx;
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let real_name = match self.recipes.get(idx) {
                            Some(real) => real.name.clone(),
                            None => {
                                self.view = View::Overview;
                                return
                            },
                        };
                        ui.label(format!("Are you sure you want to delete {}?", real_name));
                        ui.spacing();
                        if ui.button("No").clicked() {
                            self.view = View::Overview;
                        }
                        if ui.button("Yes").clicked() {
                            self.recipes.remove(idx);
                            self.edit.reset();
                            self.view = View::Overview;
                        }
                    });
                } else {
                    self.view = View::Overview;
                }
                
            }
            _ => unimplemented!()
        }
    }
}

/// State holding how and what the user is searching for
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchState {
    /// The search term to match in searched text
    term: String,

    /// If the user is searching in recipe titles for the search term
    in_titles: bool,

    /// If the user is searching in recipe descriptions for the term
    in_body: bool,

    /// A sorted list of matched recipes
    matched: BTreeMap<isize, Index>,

    /// Search ingredients for term?
    in_ingredients: bool,

    /// Lowest score of string comparison that will be included
    threshold: isize,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            term: String::new(),
            in_titles: true,
            in_body: true,
            in_ingredients: false,
            threshold: 20,
            matched: BTreeMap::new()
        }
    }
}

/// Recipe edit state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditState {
    /// If the user is adding a time to the recipe
    adding_time: bool,

    /// The user-inputted time value
    time_string: String,

    /// What unit of time the user is inputting
    time_unit: TimeUnit,

    /// A list of ingredients that the user is editing
    ingredients: Vec<(String, Option<(String, IngredientAmount)>)>,

    /// What is wrong with the current edited recipe
    wrongs: Vec<EditWrong>,

    /// The name of the edited recipe
    name: String,

    /// What recipe is being edited
    editing_recipe: Option<Index>,

    /// The body of the recipe
    body: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum EditWrong {
    BadTime,
    BadAmount,
}

impl fmt::Display for EditWrong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadTime => write!(f, "Invalid time"),
            Self::BadAmount => write!(f, "Invalid ingredient amount"),
        }
    }
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            name: String::new(),
            wrongs: vec![],
            editing_recipe: None,
            ingredients: vec![],
            time_unit: TimeUnit::Minute,
            time_string: String::new(),
            adding_time: false,
            body: String::new()
        }
    }
}

impl EditState {
    /// Reset the editor state to not be editing anything
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Construct a recipe from the inputted values, returning None if an error occured
    fn to_recipe(&self) -> Result<Recipe, EditWrong> {
        Ok(Recipe {
            time: match self.adding_time {
                true => {
                    let time_val: f64 = match self.time_string.parse() {
                        Ok(val) => val,
                        Err(_) => {
                            return Err(EditWrong::BadTime)
                        }
                    };
                    let time = match self.time_unit {
                        TimeUnit::Second => time::Duration::from_secs_f64(time_val),
                        TimeUnit::Minute => time::Duration::from_secs_f64(time_val * 60.),
                        TimeUnit::Hour => time::Duration::from_secs_f64(time_val * 3600.),
                        TimeUnit::Day => time::Duration::from_secs_f64(time_val * 3600. * 24.),
                    };
                    Some(time)
                },
                false => None
            },
            ingredients: self.ingredients.iter().map(|(name, amount)| {
                if let Some((amount, unit)) = amount {
                    let amount_val: f64 = match amount.parse() {
                        Ok(val) => val,
                        Err(_) => {
                            return Err(EditWrong::BadAmount);
                        }
                    };

                    Ok(Ingredient {
                        name: name.clone(),
                        amount: match unit {
                            IngredientAmount::Count(_) => IngredientAmount::Count(amount_val as usize),
                            IngredientAmount::Mass(Mass { unit, val: _ }) => IngredientAmount::Mass(Mass::new(*unit, amount_val as f32)),
                            IngredientAmount::Volume(Volume { unit, val: _ }) => IngredientAmount::Volume(Volume::new(*unit, amount_val as f32)),
                            IngredientAmount::None => IngredientAmount::None
                        }
                    })

                } else {
                    Ok(Ingredient {
                        name: name.clone(),
                        amount: IngredientAmount::None
                    })
                }
                    
            }).collect::<Result<Vec<_>, _>>()?,
            
            name: self.name.clone(),
            body: self.body.clone(),
        })
    }

    /// Construct an `EditState` from a recipe
    pub fn from_recipe(rec: &Recipe, idx: Index) -> Self {
        Self {
            adding_time: rec.time.is_some(),
            time_string: match rec.time {
                Some(time) => (time.as_secs_f64() / 60.).to_string(),
                None => String::new()
            },
            time_unit: TimeUnit::Minute,
            name: rec.name.clone(),
            body: rec.body.clone(),
            ingredients: rec.ingredients.iter().map(|ingredient| (ingredient.name.clone(), match ingredient.amount {
                IngredientAmount::None => None,
                IngredientAmount::Count(num) => Some((num.to_string(), IngredientAmount::Count(0))),
                IngredientAmount::Mass(Mass { val, unit }) => Some((val.to_string(), IngredientAmount::Mass(Mass { val: 0., unit}))),
                IngredientAmount::Volume(Volume { val, unit }) => Some((val.to_string(), IngredientAmount::Volume(Volume { val: 0., unit}))),
            })).collect::<Vec<_>>(),
            editing_recipe: Some(idx),
            wrongs: vec![]
        }
    }
}


/// What screen the user is currently seeing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    Overview,
    Search,
    AreYouSure,
    EditRecipe,
    Settings,
}
