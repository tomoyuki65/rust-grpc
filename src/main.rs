// モジュールのインポート
mod configs;
mod contexts;
mod errors;
mod interceptors;
mod loggers;
mod middleware;
mod repositories;
mod routers;
mod server;
mod services;
mod usecases;
use configs::config::get_config;
use loggers::logger::init_logger;
use routers::router::router;

#[tokio::main]
async fn main() {
    // 環境変数取得
    let config = get_config();

    // ロガーの初期化
    init_logger();

    // サーバー起動のログ出力
    log::info!("[ENV={}] Start rust_grpc !!", config.env);

    // サーバー起動
    router().await.unwrap();
}
