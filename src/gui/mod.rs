pub mod data;
pub mod theme;
pub mod ui;
pub mod widgets;
pub mod handler;
mod impls;

use self::data::screen::AppScreen;
use druid::Selector;
pub use ui::root_widget;

/// The visually-appealing golden ratio
pub const GOLDEN_RATIO: f64 = 1.61803;

/// Change the currently visisble screen command
pub const CHANGE_SCREEN: Selector<AppScreen> = Selector::new("recipier.change-screen");

