// gen from: cargo build
fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["qic.proto"], &["./proto/"])
        .unwrap();
}
