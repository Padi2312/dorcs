mod dorcs;

use dorcs::dorcser::Dorcser;

fn main() {
    let dorcser = Dorcser::new();
    dorcser.build();
}
