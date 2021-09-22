use druid::{AppLauncher, WindowDesc, widget::{Align, Button, Flex, Label, Widget, prelude::*}};
use self_update::cargo_crate_version;
use semver::Version;

/// Get a widget for an update
fn update_widget(new_version: Version) -> impl Widget<()> {
        let label = Label::new(format!("An update to the application is availible (new: {}/ current: {})", new_version, cargo_crate_version!()));
        let ok = Button::new("Install update")
            .on_click(move |ctx, _, _env| {
                ctx.window().close();
            });
        let cancel = Button::new("Continue without updating");
        let buttons = Flex::row()
            .with_child(cancel)
            .with_spacer(20.0)
            .with_child(ok);
        let panel = Flex::column()
            .with_child(label)
            .with_default_spacer()
            .with_child(buttons);
        Align::centered(panel)
}

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

    let max = match releases.iter().map(|rel| rel.version.parse::<Version>()).max_by(|this, next| match (this, next) {
        (Ok(this), Ok(next)) => this.cmp(next),
        (Ok(_), _) => std::cmp::Ordering::Greater,
        (_, Ok(_)) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal
    }) {
        Some(Ok(max)) => max,
        _ => return Ok(())
    };
    
    //Check if a newer version is availible
    if max > cargo_crate_version!().parse::<Version>().unwrap() {
        let desc = WindowDesc::new(move || update_widget(max))
            .title("Update Recommended")
            .window_size((600f64, 800f64));
        AppLauncher::with_window(desc)
            .launch(()).unwrap_or_else(|e| panic!("{}", e));
    }   
    Ok(())
}