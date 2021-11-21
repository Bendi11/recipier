//! Autoupdate functionality checking for new github releases and prompting the user to install them

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, ExtEventSink, Lens, Widget, WidgetExt, WindowDesc};
use semver::Version;
use serde_json::Value;
use ureq::Agent;
use thiserror::Error;
use std::rc::Rc;
use parking_lot::Mutex;

use super::VERSION;

/// The accepted github API content type
const ACCEPT_TYPE: &str = "application/vnd.github.v3+json";

/// Check for new github releases and prompt the user to update in a separate window if there is a new one
pub fn autoupdate(sender: ExtEventSink) -> Result<(), UpdateError> {
    let client = Agent::new();

    let response = client.get("https://api.github.com/repos/bendi11/recipier/releases/latest")
        .set("accept", ACCEPT_TYPE)
        .call()?;

    if response.status() != 200 {
        return Err(UpdateError::LatestReleaseNotFound)
    }
    
    let release: Value = response.into_json()?;

    let release_version = release.get("name")
        .ok_or_else(|| UpdateError::InvalidJsonResponse("'name' field missing from release object"))?
        .as_str()
        .ok_or_else(|| UpdateError::InvalidJsonResponse("'name' field of release object is not a string"))?
        .parse::<Version>()
        .map_err(|_| UpdateError::InvalidJsonResponse("Latest release's name is not a valid semver version!"))?;

    if release_version > *VERSION {
        log::trace!("Github release version {} is higher than current {}, prompting to update...", release_version, *VERSION);

        let update = Rc::new(Mutex::new(false));

        let state = PromptState {
            update: update.clone(),
            new_version: release_version
        };

        let dialog_window = WindowDesc::new(prompt_widget)
            .title("Update")
            .resizable(false)
            .show_titlebar(false)
            .window_size((480., 100.));

        AppLauncher::with_window(dialog_window)
            .configure_env(|env, _state| super::gui::theme::set(env))
            .launch(state)
            .map_err(|e| UpdateError::DialogFailed(e))?;
        
        if *update.lock() == true {
            
        }
    }

    Ok(())
}

/// Generate the root widget for the restart and update prompt
fn prompt_widget() -> impl Widget<PromptState> {
    Flex::column()
        .with_default_spacer()
        .with_child(Label::dynamic(|state: &PromptState, _| 
            format!("Recipier version {} is available to update, would you like to update and restart?", state.new_version))
        )
        .with_default_spacer()
        .with_child(Flex::row()
            .with_child(Button::new("Don't Update")
                .on_click(|_ctx, data: &mut PromptState, _| *data.update.lock() = false)
            )
            .with_flex_spacer(1.0)
            .with_child(Button::new("Restart and Update")
                .on_click(|_ctx, data: &mut PromptState, _| *data.update.lock() = true)
            )
            .padding((10., 0.))
        )
        .with_default_spacer()
}

/// App state for the simple restart and update prompt
#[derive(Clone, Debug, Data, Lens,)]
struct PromptState {
    /// If the user wants to update
    update: Rc<Mutex<bool>>,

    /// The version that the user can upgrade to
    #[data(same_fn = "PartialEq::eq")]
    new_version: Version,
}   

/// Enumeration defining all errors that can occur when autoupdating
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("HTTP request error: {}", .0)]
    Ureq(#[from] ureq::Error),

    #[error("Input/output error: {}", .0)]
    Io(#[from] std::io::Error),

    #[error("Release list for github repository not found by API")]
    LatestReleaseNotFound,

    #[error("Github API JSON response was invalid: {}", .0)]
    InvalidJsonResponse(&'static str),

    #[error("Failed to start dialog window: {}", .0)]
    DialogFailed(#[from] druid::PlatformError),
}