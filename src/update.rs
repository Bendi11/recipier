//! Autoupdate functionality checking for new github releases and prompting the user to install them

use druid::commands::CLOSE_WINDOW;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, ExtEventSink, Lens, Target, Widget, WidgetExt, WindowDesc};
use parking_lot::Mutex;
use semver::Version;
use serde_json::Value;
use std::env::consts::EXE_SUFFIX;
use std::{fs, path, rc::Rc};
use thiserror::Error;
use ureq::Agent;
use zip::ZipArchive;

use super::VERSION;

/// The accepted github API content type
const ACCEPT_TYPE: &str = "application/vnd.github.v3+json";

/// A structure holding platform name and word size from a github release asset filename formatted as
/// {os}-x{width}.zip
#[derive(Clone, Copy, Debug)]
struct ReleaseAsset<'a> {
    /// The operating system string of this release asset
    os: &'a str,
    /// The width, 32 for x86 and 64 for x64
    width: u8,
    /// The ID of this asset
    id: u64,
}

impl<'a> ReleaseAsset<'a> {
    /// Parse a release asset filename into a release asset structure
    pub fn parse(asset: &'a Value) -> Option<Self> {
        let file = asset.get("name")?.as_str()?;
        let name = path::Path::new(file).file_name()?.to_str()?;
        let mut parts = name.split('-');
        let os = parts.next()?;
        let width = match parts.next()?.trim_start_matches('x').parse::<u8>().ok()? {
            86 => 32,
            other => other,
        };

        Some(Self {
            os,
            width,
            id: asset.get("id")?.as_u64()?,
        })
    }
}

/// Check for new github releases and prompt the user to update in a separate window if there is a new one
pub fn autoupdate(sender: ExtEventSink) -> Result<(), UpdateError> {
    let client = Agent::new();

    let response = client
        .get("https://api.github.com/repos/bendi11/recipier/releases/latest")
        .set("accept", ACCEPT_TYPE)
        .call()?;

    if response.status() != 200 {
        return Err(UpdateError::LatestReleaseNotFound);
    }

    let release: Value = response.into_json()?;

    let release_version = release
        .get("name")
        .ok_or(
            UpdateError::InvalidJsonResponse("'name' field missing from release object")
        )?
        .as_str()
        .ok_or(
            UpdateError::InvalidJsonResponse("'name' field of release object is not a string")
        )?
        .parse::<Version>()
        .map_err(|_| {
            UpdateError::InvalidJsonResponse("Latest release's name is not a valid semver version!")
        })?;

    if release_version > *VERSION {
        let release_assets = release
            .get("assets")
            .ok_or(
                UpdateError::InvalidJsonResponse("'assets' field missing from release object")
            )?
            .as_array()
            .ok_or(
                UpdateError::InvalidJsonResponse("assets field of release object is not an array")
            )?;

        let mut matching_asset = None;
        //Find a release asset matching our platform and word size
        for asset in release_assets.iter().filter_map(ReleaseAsset::parse) {
            if asset.os == std::env::consts::OS
                && asset.width == (std::mem::size_of::<usize>() * 8) as u8
            {
                matching_asset = Some(asset);
                break;
            }
        }

        let matching_asset = matching_asset.ok_or(UpdateError::NoMatchingAsset)?;
        log::trace!(
            "Github release version {} is higher than current {}, prompting to update",
            release_version,
            *VERSION
        );

        let update = Rc::new(Mutex::new(false));
        let state = PromptState {
            update: update.clone(),
            new_version: release_version.clone(),
        };

        let dialog_window = WindowDesc::new(prompt_widget)
            .title("Update")
            .resizable(false)
            .show_titlebar(false)
            .window_size((480., 100.));

        AppLauncher::with_window(dialog_window)
            .configure_env(|env, _state| super::gui::theme::set(env))
            .launch(state)
            .map_err(UpdateError::DialogFailed)?;

        if *update.lock() {
            sender.submit_command(CLOSE_WINDOW, (), Target::Global)?;
            let mut temp = tempfile::tempfile()?;
            let mut response = client
                .get(&*format!(
                    "https://api.github.com/repos/bendi11/recipier/releases/assets/{}",
                    matching_asset.id
                ))
                .set("Accept", "application/octet-stream")
                .call()?
                .into_reader();
            std::io::copy(&mut response, &mut temp)?;
            drop(response);

            let mut zipfile = ZipArchive::new(&mut temp)?;
            zipfile.extract(release_version.to_string())?;
            log::trace!("Unpacked zip archive to application directory");

            drop(zipfile);
            drop(temp);

            let main_hardlink = format!("./reciper{}", EXE_SUFFIX); //The path to the main application hardlink
            fs::remove_file(&main_hardlink)?; //Remove the old hard link
            fs::hard_link(
                format!("./{}/recipier{}", release_version, EXE_SUFFIX),
                &main_hardlink,
            )?;

            log::trace!("Created all links, restarting...");
            std::process::Command::new(main_hardlink).spawn()?;
        }
    }

    Ok(())
}

/// Generate the root widget for the restart and update prompt
fn prompt_widget() -> impl Widget<PromptState> {
    Flex::column()
        .with_default_spacer()
        .with_child(Label::dynamic(|state: &PromptState, _| {
            format!(
                "Recipier version {} is available to update, would you like to update and restart?",
                state.new_version
            )
        }))
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Don't Update")
                        .on_click(|_ctx, data: &mut PromptState, _| *data.update.lock() = false),
                )
                .with_flex_spacer(1.0)
                .with_child(
                    Button::new("Restart and Update")
                        .on_click(|_ctx, data: &mut PromptState, _| *data.update.lock() = true),
                )
                .padding((10., 0.)),
        )
        .with_default_spacer()
}

/// App state for the simple restart and update prompt
#[derive(Clone, Debug, Data, Lens)]
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

    #[error("Error decompressing zip file: {}", .0)]
    Zip(#[from] zip::result::ZipError),

    #[error("Release list for github repository not found by API")]
    LatestReleaseNotFound,

    #[error("Github API JSON response was invalid: {}", .0)]
    InvalidJsonResponse(&'static str),

    #[error("Failed to start dialog window: {}", .0)]
    DialogFailed(#[from] druid::PlatformError),

    #[error("No release asset matching os and word size found")]
    NoMatchingAsset,

    #[error("Failed to send command to main window: {}", .0)]
    EventSendError(#[from] druid::ExtEventError),
}
