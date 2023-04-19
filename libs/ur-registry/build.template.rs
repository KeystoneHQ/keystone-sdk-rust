fn main() {
    prost_build::Config::new().btree_map(&["."])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("src/pb")
        .compile_protos(&["base.proto"], &["./proto/"])
        .unwrap();
}
