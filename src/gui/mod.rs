use eframe::{
    egui::{self, TextEdit, Widget, Key, TextStyle, Color32}, 
    epi::App
};
use generational_arena::Arena;
use crate::{
    recipe::Recipe
};


/// Recipe application holding all state for the GUI and all recipes
#[derive(Clone, Debug)]
pub struct RecipeApp {
    /// An allocator for recipes, so that recipes can be addressed by handle instead of
    /// needing smart pointers or references
    recipes: Arena<Recipe>,

    /// What screen is being displayed
    view: View,

    /// What text is in the search bar
    search_text: String,
}

impl RecipeApp {
    pub fn new() -> Self {
        Self {
            recipes: Arena::new(),
            view: View::Overview,
            search_text: String::new()
        }
    }
}

impl App for RecipeApp {
    fn name(&self) -> &str {
        "recipier"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        match self.view {
            View::Overview => {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    let search = TextEdit::singleline(&mut self.search_text)
                        .hint_text("search")
                        .text_style(TextStyle::Small);
                    if search.ui(ui).lost_focus() && ctx.input().key_pressed(Key::Enter) {
                        self.view = View::Search;
                    }
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Recipes");
                });
            },  
            _ => unimplemented!()
        }
    }
}

/// What screen the user is currently seeing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Overview,
    Search,
    Settings
}
