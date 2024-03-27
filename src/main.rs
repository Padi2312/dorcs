mod dorcs;

use dorcs::{dorcser::Dorcser, wizard};

fn main() {
    wizard::setup();
    let dorcser = Dorcser::new();
    dorcser.build();
}
