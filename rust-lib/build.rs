use std::process::Command;

fn main() {
    let udl_file = "./src/rust-lib.udl";
    let out_dir = "./bindings/";
    uniffi_build::generate_scaffolding(udl_file).unwrap();
    Command::new("uniffi-bindgen-cs").arg("--out-dir").arg(out_dir).arg(udl_file).output().expect("Failed when generating C# bindings");
}