//! Autoupdate functionality checking for new github releases and prompting the user to install them

use druid::commands::CLOSE_ALL_WINDOWS;
use druid::{ExtEventSink, Target};
use semver::Version;
use serde_json::Value;
use std::env::consts::EXE_SUFFIX;
use std::sync::mpsc;
use std::fs;
use thiserror::Error;
use ureq::Agent;
use zip::ZipArchive;

use crate::TARGET_TRIPLE;
use crate::gui::SHOW_UPDATE_DIALOG;

use super::VERSION;

/// The accepted github API content type
const ACCEPT_TYPE: &str = "application/vnd.github.v3+json";

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
        .get("tag_name")
        .ok_or(UpdateError::InvalidJsonResponse(
            "'tag_name' field missing from release object",
        ))?
        .as_str()
        .ok_or(UpdateError::InvalidJsonResponse(
            "'tag_name' field of release object is not a string",
        ))?
        .trim_start_matches('v')
        .parse::<Version>()
        .map_err(|_| {
            UpdateError::InvalidJsonResponse("Latest release's name is not a valid semver version!")
        })?;

    if release_version > *VERSION {
        let release_assets = release
            .get("assets")
            .ok_or(UpdateError::InvalidJsonResponse(
                "'assets' field missing from release object",
            ))?
            .as_array()
            .ok_or(UpdateError::InvalidJsonResponse(
                "assets field of release object is not an array",
            ))?;

        let mut matching_asset = None;
        //Find a release asset matching our platform and word size
        for (name, id) in release_assets.iter().filter_map(|v| Some((v.get("name")?.as_str()?.split('.').next()?, v.get("id")?.as_u64()?))) {
            let triple = name.split_once('-').map(|v| v.1);
            if let Some(triple) = triple {
                if triple == TARGET_TRIPLE {
                    matching_asset = Some(id);
                }
            }
        }

        let matching_asset = matching_asset.ok_or(UpdateError::NoMatchingAsset)?;
        log::trace!(
            "Github release version {} is higher than current {}, prompting to update",
            release_version,
            *VERSION
        );

        let (choice_tx, choice_rx) = mpsc::channel();

        if let Err(e) = sender.submit_command(SHOW_UPDATE_DIALOG, choice_tx, Target::Global) {
            log::error!("Failed to submit update dialog prompt command: {}", e);
            return Err(UpdateError::EventSendError(e))
        }
        match choice_rx.recv() {
            Ok(choice) => if choice {
                sender.submit_command(CLOSE_ALL_WINDOWS, (), Target::Global)?;
                let mut temp = tempfile::tempfile()?;
                let mut response = client
                    .get(&*format!(
                        "https://api.github.com/repos/bendi11/recipier/releases/assets/{}",
                        matching_asset
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
            },
            Err(e) => {
                log::error!("Failed to receive an update dialog response: {}", e);
            }
        }
    }

    Ok(())
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
