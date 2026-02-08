fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(true)
        .compile_protos(&["src/proto/blog.proto"], &["src/proto"])?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
