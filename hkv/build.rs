// gen from: cargo build
fn main() {
    let mut config = prost_build::Config::new();
    // 生成 bytes::Bytes 给 bytes 属性的字段，而不是使用缺省的 Vec<u8> 
    config.bytes(&["."]);
    // 给 .(当前包全部) 加上 PartialOrd derive
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config.out_dir("src/pb")
        .compile_protos(&["qic.proto"], &["./proto/"])
        .unwrap();
}
