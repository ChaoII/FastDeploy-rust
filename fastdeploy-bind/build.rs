extern crate bindgen;

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use cmake::Config;

fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}

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
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let mut config = Config::new(fastdeploy_src_dir());
    config
        .generator("Visual Studio 17 2022")
        // .define("CMAKE_MAKE_PROGRAM", "C:/software/CLion 2022.3.2/bin/ninja/win/x64/ninja.exe")
        .define("OPENCV_DIRECTORY", "E:/develop/opencv4.9/build/x64/vc16/lib")
        .define("ENABLE_VISION", "ON")
        .define("ENABLE_ORT_BACKEND", "ON")
        .define("BUILD_PADDLE2ONNX", "ON")
        .define("WITH_CAPI", "ON")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_VERBOSE_MAKEFILE", "ON")
        .build_arg("-j18")
        .build_arg("-v")
        .profile("Release");
    println!("============{}=============", config.get_profile());

    let destination = config.build();

    // 输出构建完成信息
    println!("cargo:rerun-if-changed=src/lib.rs");

    if target.contains("window") && !target.contains("gnu") {
        println!(
            "cargo:rustc-link-search={}",
            out.join("build").join("Release").display()
        );
    } else {
        println!("cargo:rustc-link-search={}", out.join("build").display());
    }
    println!("cargo:rustc-link-search=native={}", destination.display());
    println!("cargo:rustc-link-lib=fastdeploy");
    Ok(())
}

fn build_fastdeploy() -> Vec<PathBuf> {
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
            vec![output_dir().join("include")]
        };
    return include_paths;
}

fn main() {
    env::set_var("OUT_DIR", "./");
    env::set_var("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG", "true");
    // let bindings = build_fastdeploy();
    println!("============fastdeploy_dir================");
    let include_paths = "E:/FastDeploy/build/install_mnn/include";
    println!("inculde path{:?}", include_paths);
    println!("cargo:rustc-link-search=native={}", "E:/FastDeploy/build/install_mnn/lib");
    println!("cargo:rustc-link-search=native={}", "E:/FastDeploy-rust/target/debug");
    println!("cargo:rustc-link-lib=fastdeploy");
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", &include_paths))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(output_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
