extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {

    let mut dst: PathBuf = Config::new("discord-rpc")
                // .generator("Visual Studio 15 2017 Win64")
                .profile("Release")
                .build();

    dst.push("lib");
    println!(r"cargo:rustc-link-search=native={}", dst.display());
    println!(r"cargo:rustc-link-lib=static=discord-rpc");
}
