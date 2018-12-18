//extern crate pkg_config;

extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    /*
    match pkg_config::probe_library("smbclient") {
        Ok(_) => {
            if cfg!(target_os = "macos") {
                //println!("cargo:rustc-flags=-L /usr/local/lib -l smbclient");
                println!("cargo:rustc-link-lib=static=smbclient")
            } else {
                //println!("cargo:rustc-flags=-l smbclient");
                println!("cargo:rustc-link-lib=smbclient")
            }
        }
        Err(e) => {
            println!(
                "error: SMB Client library not found! Probably libsmbclient is not installed."
            );
            panic!("{}", e);
        }
    };
    */
    //println!("cargo:rustc-link-lib=smbclient");
    println!("cargo:rustc-flags=-L /usr/local/samba -l smbclient");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
