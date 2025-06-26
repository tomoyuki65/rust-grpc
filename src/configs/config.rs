use envy;
use serde::Deserialize;

// 環境変数のデフォルト値を返す関数
fn default_env() -> String {
    "local".to_string()
}

fn default_grpc_port() -> u16 {
    50051
}

fn default_rust_log() -> String {
    "info".to_string()
}

// 環境変数の構造体
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
    #[allow(dead_code)]
    #[serde(default = "default_rust_log")]
    pub rust_log: String,
}

// 環境変数を返す関数
pub fn get_config() -> Config {
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(err) => {
            println!("環境変数の初期化エラー: {}", err);

            // 環境変数にデフォルト値を設定して返す
            Config {
                env: default_env(),
                grpc_port: default_grpc_port(),
                rust_log: default_rust_log(),
            }
        }
    }
}
