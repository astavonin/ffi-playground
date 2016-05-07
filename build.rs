use std::process::Command;
use std::path::Path;
use std::fs;
use std::env;

pub fn main() {
    let cpplib_bin_name = "libcpp2rs.dylib";
    let cpplib_str = "./cpplib";
    let cpplib_path = Path::new(cpplib_str);

    match fs::metadata(format!("{}/Makefile", cpplib_str)) {
        Err(..) => {
            Command::new("cmake").args(&["CMakeLists.txt"])
                .current_dir(&cpplib_path).status().unwrap();
        }
        Ok(..) => { }
    }
    Command::new("make").current_dir(&cpplib_path).status().unwrap();

    let out_dir = format!("{}/{}",
                          env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string()),
                          env::var("PROFILE").unwrap());

    let src_file = format!("{}/bin/{}", cpplib_str, cpplib_bin_name);
    let dst_file = format!("{}/{}", out_dir, cpplib_bin_name);
    fs::copy(src_file, dst_file).unwrap();

    println!("cargo:rustc-link-search=native={}/bin", cpplib_str);
}

