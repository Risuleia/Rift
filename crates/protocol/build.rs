fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;

    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    prost_build::Config::new().compile_protos(&["proto/rift.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/rift.proto");

    Ok(())
}
