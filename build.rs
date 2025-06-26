use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コンフィグ設定を定義
    let files = &["proto/sample/sample.proto"];
    let includes = &[
        "proto",
        "../root/go/pkg/mod/github.com/envoyproxy/protoc-gen-validate@v1.2.1",
    ];
    let file_descriptor_set_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
        .join("sample_descriptor.bin");
    let mut config = {
        let mut c = prost_build::Config::new();
        c.service_generator(
            tonic_build::configure()
                .message_attribute(".", "#[derive(::prost_validate::Validator)]")
                .service_generator(),
        );
        c
    };

    // コンフィグ設定にバリデーションを適用
    prost_validate_build::Builder::new()
        .file_descriptor_set_path(file_descriptor_set_path)
        .configure(&mut config, files, includes)?;

    // コンフィング設定からコード生成
    config.compile_protos(files, includes)?;

    Ok(())
}
