use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("blog_descriptor.bin"))
        .compile_protos(&["src/proto/blog.proto"], &["src/proto"])?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
