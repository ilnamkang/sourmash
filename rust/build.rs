extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.parse.parse_deps = true;
    config.parse.include = Some(vec!["sourmash".to_owned()]);
    config.parse.expand = vec!["sourmash".to_owned()];
    config.language = cbindgen::Language::C;
    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file("target/sourmash.h");
}
