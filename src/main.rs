pub mod config;
mod core;
pub mod navigation;

use core::dorcs::Dorcs;

fn main() {
    Dorcs::new().init();
}
