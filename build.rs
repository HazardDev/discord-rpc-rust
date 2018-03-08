extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // let bindings = bindgen::Builder::default()
    //     .header(".\\discord-rpc\\include\\discord_rpc.h")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from("src");
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    println!(r"cargo:rustc-link-search=discord-rpc\lib\");
}
