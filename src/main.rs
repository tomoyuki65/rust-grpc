// モジュールのインポート
mod contexts;
mod interceptors;
mod loggers;
mod middleware;
mod routers;
mod server;
mod usecases;
use loggers::logger::init_logger;
use routers::router::router;

#[tokio::main]
async fn main() {
    // ロガーの初期化
    init_logger();

    // サーバー起動のログ出力
    log::info!("Start rust_grpc !!");
    // log::info!("Start rust_api (ENV:{}) !!", config.env);

    // サーバー起動
    router().await.unwrap();
}
