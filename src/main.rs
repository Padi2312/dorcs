mod dorcs;

use dorcs::config::Config;
use dorcs::generator::Generator;

fn main() {
    let config: Config = Config::load();
    let generator = Generator::new(config);
    generator.generate_docs();
}
