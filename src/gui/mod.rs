use eframe::{
    egui::{self, Key, TextEdit, Visuals, Widget, Label, Button, Color32}, 
    epi::App
};
use serde::{Serialize, Deserialize};
use std::{collections::BTreeMap, fmt, time};
use generational_arena::{Arena, Index};
use crate::{measure::TimeUnit, recipe::{IngredientAmount, Recipe}};

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
    matched_recipes: Option<BTreeMap<isize, Index>>,

    /// Edit state
    edit: EditState,

    /// What text is in the search bar
    search_text: String,
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
    ingredients: Vec<(String, IngredientAmount)>,

    /// What is wrong with the current edited recipe
    wrongs: Vec<EditWrong>,

    /// What recipe is being edited
    editing_recipe: Option<(Index, Recipe)>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum EditWrong {
    BadTime,
}

impl fmt::Display for EditWrong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadTime => write!(f, "Invalid time")
        }
    }
}

impl EditState {
    /// Reset the editor state to not be editing anything
    pub fn reset(&mut self) {
        self.adding_time = false;
        self.time_string = String::new();
        self.time_unit = TimeUnit::Minute;
        self.editing_recipe = None;
        self.wrongs = vec![];
        self.ingredients = vec![];
    }
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
            edit: EditState {
                adding_time: false,
                editing_recipe: None,
                time_string: String::new(),
                time_unit: TimeUnit::Minute,
                wrongs: vec![],
                ingredients: vec![]
            },
            search_text: String::new(),
            matched_recipes: None,
            
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
                    let new = self.recipes.insert(Recipe::default());
                    self.edit.editing_recipe = Some((new, Recipe::default()));
                    self.view = View::EditRecipe;
                } 
            });

            let search = TextEdit::singleline(&mut self.search_text)
                .hint_text("search");
                ui.centered_and_justified(|ui| search.ui(ui));
            if ctx.input().key_pressed(Key::Enter) {
                const SEARCH_THRESHOLD: isize = 0;
                self.matched_recipes = Some({
                    let mut matches = BTreeMap::new();
                    for (idx, recipe) in self.recipes.iter() {
                        let max = match sublime_fuzzy::best_match(self.search_text.as_str(), recipe.name.as_str()) {
                            Some(score) => score.score(),
                            None => continue,
                        }.max(match sublime_fuzzy::best_match(self.search_text.as_str(), recipe.name.as_str()) {
                            Some(score) => score.score(),
                            None => continue,
                        });
                        if max >= SEARCH_THRESHOLD {
                            matches.insert(max, idx);
                        }
                    }
                    matches
                });
                self.view = View::Search;
            }
        });
    }

    /// Show a widget for a recipe
    fn show_recipe(ui: &mut egui::Ui, idx: Index, recipes: &Arena<Recipe>, view: &mut View, editing_recipe: &mut Option<(Index, Recipe)>) {
        let recipe = recipes.get(idx).unwrap();
        ui.columns(3, |ui| {
            ui[0].heading(&recipe.name);
            ui[0].add_space(10.);

            if ui[1].button("Edit").clicked() {
                *view = View::EditRecipe;
                *editing_recipe = Some((idx, recipe.clone()));
            }
            if ui[2].button("Delete").clicked() {
                *view = View::AreYouSure;
                *editing_recipe = Some((idx, recipe.clone()));
            }
        });

        
        ui.collapsing("Ingredients", |ui| {
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

impl App for RecipeApp {
    fn name(&self) -> &str {
        "recipier"
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
                            Self::show_recipe(ui, idx, &self.recipes, &mut self.view, &mut self.edit.editing_recipe);
                            ui.separator();
                        }
                    }
                });
            },  
            View::Search => {
                self.top_menubar(ctx);

                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(matched) = self.matched_recipes.as_ref() {
                        for (_, idx) in matched.iter() {
                            let idx = *idx;
                            Self::show_recipe(ui, idx, &self.recipes, &mut self.view, &mut self.edit.editing_recipe);
                            ui.separator();
                        }
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
                            if let Some((idx, recipe)) = self.edit.editing_recipe.as_ref() {
                                let mut recipe = recipe.clone();
                                let mut ok = true;
                                if self.edit.adding_time {
                                    let time_val: f64 = match self.edit.time_string.parse() {
                                        Ok(val) => val,
                                        Err(_) => {
                                            self.edit.wrongs.push(EditWrong::BadTime);
                                            ok = false;
                                            0.
                                        }
                                    };
                                    let time = match self.edit.time_unit {
                                        TimeUnit::Second => time::Duration::from_secs_f64(time_val),
                                        TimeUnit::Minute => time::Duration::from_secs_f64(time_val * 60.),
                                        TimeUnit::Hour => time::Duration::from_secs_f64(time_val * 3600.),
                                        TimeUnit::Day => time::Duration::from_secs_f64(time_val * 3600. * 24.),
                                    };
                                    recipe.time = Some(time);
                                }
                                if ok {
                                    *self.recipes.get_mut(*idx).unwrap() = recipe;
                                    self.view = View::Overview;
                                }   
                            }
                        }
                    })
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some((_, ref mut recipe)) = self.edit.editing_recipe {
                        for wrong in self.edit.wrongs.iter() {
                            ui.colored_label(Color32::from_rgb(255, 61, 61), wrong.to_string());
                        }
                        ui.small("Title");
                        ui.text_edit_singleline(&mut recipe.name).on_hover_text("Edit the title of the recipe");
                        ui.separator();

                        ui.checkbox(&mut self.edit.adding_time, "Add Time");
                        if self.edit.adding_time {
                            ui.text_edit_singleline(&mut self.edit.time_string);
                            ui.small("Time");
                            egui::ComboBox::from_label("unit")
                                .selected_text(self.edit.time_unit.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Second, "second");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Minute, "minute");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Hour, "hour");
                                    ui.selectable_value(&mut self.edit.time_unit, TimeUnit::Day, "day");
                                });
                        }
                        ui.separator();
                        
                    }
                    
                });
            },
            View::AreYouSure => {
                if let Some((idx, _)) = self.edit.editing_recipe.as_ref() {
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

/// What screen the user is currently seeing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    Overview,
    Search,
    AreYouSure,
    EditRecipe,
    Settings,
}
