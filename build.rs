extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!(r"cargo:rustc-link-search=discord-rpc\lib\");
}
