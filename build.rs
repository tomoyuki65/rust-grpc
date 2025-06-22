use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
                .join("sample_descriptor.bin"),
        )
        .compile_protos(&["proto/sample/sample.proto"], &["proto/sample"])?;

    Ok(())
}
