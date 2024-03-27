extern crate dorcs_json_schema;
use schemars::schema_for;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

fn generate_json_schema() {
    let schema = schema_for!(dorcs_json_schema::ConfigJsonSchema);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    let schema_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("json-schema/")
        .join("dorcs.config.schema.json");
    std::fs::write(schema_path, schema_json).unwrap();
}

fn npm_version() {
    let npm_version = Command::new(NPM)
        .arg("-v")
        .output()
        .expect("Failed to run npm -v");
    if !npm_version.status.success() {
        panic!(
            "npm -v failed with output: {:?}",
            String::from_utf8_lossy(&npm_version.stderr)
        );
    }
}

fn npm_install(frontend_dir: &Path) {
    let npm_install_output = Command::new(NPM)
        .arg("install")
        .current_dir(&frontend_dir)
        .output()
        .expect("Failed to execute npm install");
    if !npm_install_output.status.success() {
        panic!(
            "npm install failed with output: {:?}",
            String::from_utf8_lossy(&npm_install_output.stderr)
        );
    }
}

fn npm_build(frontend_dir: &Path) {
    let npm_build_output = Command::new(NPM)
        .arg("run")
        .arg("build")
        .current_dir(&frontend_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run npm run build");
    if !npm_build_output.status.success() {
        panic!(
            "npm install failed with output: {:?}",
            String::from_utf8_lossy(&npm_build_output.stderr)
        );
    }
}

fn main() {
    let frontend_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("frontend");

    npm_version();
    npm_install(&frontend_dir);
    npm_build(&frontend_dir);
    generate_json_schema();
}
