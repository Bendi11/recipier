#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico,assets/icon.png");
    if let Err(e) = winres::WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()
    {
        eprintln!("Error adding windows resource file to executable: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {}
