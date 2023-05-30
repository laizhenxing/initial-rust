use std::process::Command;

fn main() {
    // compile proto files
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/initial.proto"], &["protos"])
        .unwrap();

    // format generated files
    Command::new("cargo").args(&["fmt"]).status().unwrap();

    // rerun build.rs if proto files change
    println!("cargo:rerun-if-changed=protos/initial.proto");
}
