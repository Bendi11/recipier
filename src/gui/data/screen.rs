//! What screen the GUI is currently viewing

use druid::Data;
use serde::{Deserialize, Serialize};

/// All viewable screens in the GUI, stored in the [AppState](super::AppState) struct
#[derive(Clone, Copy, Debug, PartialEq, Eq, Data, Serialize, Deserialize)]
pub enum AppScreen {
    /// The main screen allowing the user to search and view recent recipes
    Home,
    /// Search results screen with the results of a query
    SearchResults,
    /// The screen displaying one recipe's data
    View,
    /// Screen allowing the user to edit a recipe
    Edit,
    /// Screen displaying a simple are you sure prompt
    Delete,
}
