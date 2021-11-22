#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn package() {
    println!("cargo:rerun-if-changed=assets/icon.ico,assets/icon.png");
    if let Err(e) = winres::WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()
    {
        eprintln!("Error adding windows resource file to executable: {}", e);
    }
}

#[cfg(all(not(debug_assertions), target_os = "macos"))]
fn package() {
    std::fs::create_dir_all("./recipier.app/Contents")
        .expect("Failed to create app bundle directory");
    std::fs::create_dir_all("./recipier.app/Contents/MacOS")
        .expect("Failed to create app bundle binary directory");
    std::fs::create_dir_all("./recipier.app/Contents/Resources")
        .expect("Failed to create app bundle resources directory");

    std::fs::copy("assets/Info.plist", "./recipier.app/Contents/Info.plist")
        .expect("failed to copy Info.plist file");
    std::fs::copy(
        "assets/icon.icns",
        "./recipier.app/Contents/Resources/icon.icns",
    )
    .expect("failed to copy icon.icns file");
}

#[cfg(all(not(windows), not(target_os = "macos")))]
fn package() {}

fn main() {
    let info = rust_info::get();

    std::fs::hard_link("test/0.0.1/recipier.exe", "test/recipier.exe");
    
    println!("cargo:rustc-env=TARGET_TRIPLE={}", info.target_triple.unwrap());
    package()
}