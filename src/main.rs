// Disable console on windows
#![windows_subsystem = "windows"]
pub mod gui;
pub mod recipe;
pub mod measure;

use crate::{recipe::{Ingredient, IngredientAmount, Recipe}, measure::{Volume, VolumeUnit}};

fn main() {
    //Add panic handler for better error messages
    std::panic::set_hook(Box::new(|info| {
        eprintln!("A fatal error occurred in {}: {}", match info.location() {
            Some(loc) => loc.to_string(),
            None => "[unknown location]".to_owned()
        }, match (info.payload().downcast_ref::<&str>(), info.payload().downcast_ref::<String>()) {
            (Some(err), _) => (*err).to_owned(),
            (_, Some(err)) => err.clone(),
            (None, None) => "unknown error".to_owned()
        });
    }));
    let mut app = gui::RecipeApp::new();
    app.recipes.insert(Recipe {
        name: "Top Ramen".to_owned(),
        ingredients: Some(vec![
            Ingredient {
                name: "Top Ramen Packet".to_owned(),
                amount: IngredientAmount::Count(1)
            },
            Ingredient {
                name: "Water".to_owned(),
                amount: IngredientAmount::Volume(Volume::new(VolumeUnit::Cup, 2.))
            },
        ]),
        body: 
"- Add water to small / medium pot and bring to boil
- Remove noodle brick from packet and add to water
- Allow noodles to cook for around 3 minutes, stirring occasionally
- Remove heat and add flavor packet to noodles, ensuring that flavor spreads to noodles by stirring
- Leave for 5-10 minutes to cool and enjoy
".to_owned()
    });
    let opts = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), opts);
    
}
