use eframe::{
    egui::{self, Color32, Key, TextEdit, TextStyle, Visuals, Widget, Label, Button}, 
    epi::App
};
use generational_arena::{Arena, Index};
use crate::{
    recipe::Recipe
};


/// Recipe application holding all state for the GUI and all recipes
#[derive(Clone, Debug)]
pub struct RecipeApp {
    /// An allocator for recipes, so that recipes can be addressed by handle instead of
    /// needing smart pointers or references
    pub recipes: Arena<Recipe>,

    /// What screen is being displayed
    view: View,

    /// What recipe is being edited
    editing_recipe: Option<Index>,

    /// What text is in the search bar
    search_text: String,
}

impl RecipeApp {
    pub fn new() -> Self {
        Self {
            recipes: Arena::new(),
            view: View::Overview,
            editing_recipe: None,
            search_text: String::new()
        }
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

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.view {
            View::Overview => {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    ui.add_space(2.);

                    ui.columns(2, |ui| {
                        if ui[0].add(Button::new("Add")).clicked() {

                        } else if ui[1].add(Button::new("Edit")).clicked() {

                        }
                    });

                    let search = TextEdit::singleline(&mut self.search_text)
                        .hint_text("search");
                    if ui.centered_and_justified(|ui| search.ui(ui)).response.lost_focus() && ctx.input().key_pressed(Key::Enter) {
                        self.view = View::Search;
                    }
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.recipes.is_empty() {
                        ui.add(Label::new("No Recipes Yet!"));
                    } else {
                        for (_, recipe) in self.recipes.iter() {
                            ui.heading(&recipe.name);
                            if let Some(ingredients) = recipe.ingredients.as_ref() {
                                ui.collapsing("Ingredients", |ui| {
                                    for ingredient in ingredients.iter() {
                                        ui.label(format!("- {}", ingredient.to_string()));
                                    }
                                });
                            }
                            ui.separator();
                            let label = Label::new(&recipe.body).wrap(true);
                            ui.group(|ui| label.ui(ui));
                            
                        }
                    }
                });
            },  
            View::Search => {
                
            }
            _ => unimplemented!()
        }
    }
}

/// What screen the user is currently seeing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Overview,
    Search,
    EditRecipe,
    Settings,
}
