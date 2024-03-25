use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

fn main() {
    println!("Starting build.rs script");
    println!("Building frontend assets");
    //Check if npm and node is installed
    let npm_version = Command::new(NPM)
        .arg("-v")
        .output()
        .expect("Failed to run npm -v");
    let npm_version_output = String::from_utf8_lossy(&npm_version.stdout);
    println!("npm version: {}", npm_version_output);

    // Define the path to the frontend directory
    let frontend_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("frontend");
    // Change the current directory to the frontend directory
    let npm_install_output = Command::new(NPM)
        .arg("install")
        .current_dir(&frontend_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute npm install");
    if !npm_install_output.status.success() {
        panic!(
            "npm install failed with output: {:?}",
            String::from_utf8_lossy(&npm_install_output.stderr)
        );
    }

    // Run `npm run build` in the frontend directory
    let npm_run_build_status = Command::new(NPM)
        .arg("run")
        .arg("build")
        .current_dir(&frontend_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect("Failed to run npm run build");

    assert!(npm_run_build_status.success(), "npm run build failed");
}
