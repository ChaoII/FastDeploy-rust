extern crate bindgen;

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn output_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn fastdeploy_src_dir() -> PathBuf {
    output_dir().join("FastDeploy")
}

fn fetch() -> io::Result<()> {
    let target_dir = fastdeploy_src_dir();
    if target_dir.exists() {
        return Ok(());
    }
    let status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/PaddlePaddle/FastDeploy.git")
        .arg(&target_dir)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

fn build() -> io::Result<()> {
    let mut config = cmake::Config::new(fastdeploy_src_dir());
    config
        .out_dir("FastDeploy")
        .define("ENABLE_VISION", "ON")
        .define("ENABLE_ORT_BACKEND", "ON")
        .define("BUILD_PADDLE2ONNX", "ON")
        .define("WITH_CAPI", "ON")
        .define("CMAKE_BUILD_TYPE", "Release")
        .profile("Release");
    println!("============{}=============", config.get_profile());

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    Ok(())
}

fn build_fastdeploy() -> Vec<PathBuf> {
    println!("----------------------");
    let include_paths: Vec<PathBuf> =
        if let Ok(fastdeploy_install_dir) = env::var("FASTDEPLOY_INSTALL_DIR") {
            // use prebuild fastdeploy dir
            let dir = PathBuf::from(fastdeploy_install_dir);
            for suffix in ["lib", "lib64"].iter() {
                println!(
                    "cargo:rustc-link-search=native={}",
                    output_dir().join("lib").join(suffix).to_string_lossy()
                );
            }
            vec![dir.join("include")]
        } else {
            // fetch from github and build
            fetch().unwrap();
            build().unwrap();
            for suffix in ["lib", "lib64"].iter() {
                println!(
                    "cargo:rustc-link-search=native={}",
                    output_dir().join(suffix).to_string_lossy()
                );
            }
            vec![output_dir().join("include")]
        };
    return include_paths;
}

fn main() {
    env::set_var("FASTDEPLOY_INSTALL_DIR", "E:/FastDeploy/build/install");
    env::set_var("OUT_DIR", "./");
    let bindings = build_fastdeploy();
    println!("============fastdeploy_dir: {:?}================", bindings[0]);
    let include_paths = (&bindings[0]).to_string_lossy();
    println!("inculde path{:?}", include_paths);
    println!("cargo:rustc-link-search=native={}", "E:/FastDeploy/build/install/lib");
    println!("cargo:rustc-link-lib=fastdeploy");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", &include_paths))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(output_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
