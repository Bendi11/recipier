//! App state used throughout the GUI
use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct State {

}

impl State {
    pub fn new() -> Self {
        Self{}
    }
}