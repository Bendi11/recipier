//! Autoupdate functionality checking for new github releases and prompting the user to install them

use ureq::Agent;
use thiserror::Error;

/// The accepted github API content type
const ACCEPT_TYPE: &str = "application/vnd.github.v3+json";

/// Check for new github releases and prompt the user to update in a separate window if there is a new one
pub fn autoupdate() -> Result<(), UpdateError> {
    let client = Agent::new();

    let response = client.get("https://api.github.com/repos/bendi11/recipier/releases")
        .set("accept", ACCEPT_TYPE)
        .call()?;

    if response.status() != 200 {
        return Err(UpdateError::Github(format!("Release list request: response status is {}", response.status_text())))
    }
    

    Ok(())
}

/// Enumeration defining all errors that can occur when autoupdating
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("HTTP request error: {}", .0)]
    Ureq(#[from] ureq::Error),

    #[error("{}", .0)]
    Github(String),
}