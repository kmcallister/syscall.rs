extern crate rustc_version;
extern crate target_build_utils;
use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta()
        .expect("Failed to determine Rustc version information")
        .channel
    {
        Channel::Stable | Channel::Beta => {
            let target = target_build_utils::TargetInfo::new().unwrap();

            println!("{:#?}", target);

            let native_src = match (
                target.target_arch(),
                target.target_vendor(),
                target.target_os(),
                target.target_env(),
            ) {
                ("x86_64", _, "freebsd", _)
                | ("x86_64", _, "linux", _)
                | ("x86_64", _, "macos", _) => "native/x86_64-abi.c",
                ("x86", _, "linux", _) => "native/linux-x86.c",
                ("arm", _, "linux", _) => "native/linux-armeabi.c",
                _ => unimplemented!("Target {} is not supported on stable rust", std::env::var("TARGET").unwrap()),
            };

            cc::Build::new().file(native_src).compile("platform");
            println!("cargo:rustc-cfg=native");
        }
        Channel::Nightly | Channel::Dev => {
            println!("cargo:rustc-cfg=asm");
        }
    }
}
