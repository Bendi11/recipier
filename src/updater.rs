use druid::{AppLauncher, WindowDesc, widget::{
    Label, prelude::*, Flex, Align
}};

/// Check if a new update is availible and prompt the user to update if one is
pub fn update() -> Result<(), self_update::errors::Error> {
    //Get a list of all releases
    let releases = match self_update::backends::github::ReleaseList::configure()
        .repo_name("recipes")
        .repo_owner("bendi11")
        .build() {
            Ok(list) => list,
            Err(e) => if let self_update::errors::Error::Network(_) = e {
                return Ok(())
            } else {
                return Err(e)
            }
        }
        .fetch()?;

    Ok(())
    
}