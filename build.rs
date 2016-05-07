use std::process::Command;
use std::path::Path;
use std::fs;

pub fn main() {
    let cpplib_path = Path::new("./cpplib");

    match fs::metadata(format!("{}/Makefile", cpplib_path.to_str().unwrap())) {
        Err(..) => {
            Command::new("cmake").args(&["CMakeLists.txt"])
                .current_dir(&cpplib_path).status().unwrap();
        }
        Ok(..) => { }
    }
    Command::new("make").current_dir(&cpplib_path).status().unwrap();

    println!("cargo:rustc-link-search=native={}/bin", cpplib_path.to_str().unwrap());
}

