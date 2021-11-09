//! Configuration state for the application

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

/// Application configuration data contained in the [AppState](super::AppState) structure
#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct Config {
    /// Size of the application window in druid display points
    pub window_size: (f64, f64),
}
