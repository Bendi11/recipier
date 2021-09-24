use eframe::{
    egui::{self, Key, TextEdit, Visuals, Widget, Label, Button}, 
    epi::App
};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use generational_arena::{Arena, Index};
use crate::{
    recipe::Recipe
};

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

    /// What recipe is being edited
    editing_recipe: Option<Index>,

    /// What text is in the search bar
    search_text: String,
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
            editing_recipe: None,
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
                    self.editing_recipe = Some(new);
                    self.view = View::EditRecipe;
                } 
            });

            let search = TextEdit::singleline(&mut self.search_text)
                .hint_text("search");
                ui.centered_and_justified(|ui| search.ui(ui));
            if ctx.input().key_pressed(Key::Enter) {
                const SEARCH_THRESHOLD: isize = 20;
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

    fn show_recipe(ui: &mut egui::Ui, recipe: &Recipe) {
        ui.heading(&recipe.name);
        if let Some(ingredients) = recipe.ingredients.as_ref() {
            ui.collapsing("Ingredients", |ui| {
                for ingredient in ingredients.iter() {
                    ui.label(format!("- {}", ingredient.to_string()));
                }
            });
        }
        ui.separator();
        if let Some(time) = recipe.time {
            ui.label(humantime::format_duration(time).to_string());
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
            serde_json::to_writer(save, &self);
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
                        for (_, recipe) in self.recipes.iter() {
                            Self::show_recipe(ui, recipe);
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
                            let recipe = self.recipes.get(*idx).unwrap();
                            Self::show_recipe(ui, recipe);
                            ui.separator();
                        }
                    }
                });

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
    EditRecipe,
    Settings,
}
