// tonic
use tonic::{Response, Status};

// tokio
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tokio_stream::wrappers::ReceiverStream;

// 共通コンテキスト
use crate::contexts::context::Context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 実行するユースケースの構造体
pub struct SampleHelloServerStreamUsecase {}

// ストリーミング用の型定義
type HelloServerStreamStream = ReceiverStream<Result<sample_proto::HelloServerStreamResponseBody, Status>>;

impl SampleHelloServerStreamUsecase {
    pub async fn exec(
        &self,
        ctx: Context,
        req_body: sample_proto::HelloServerStreamRequestBody,
    ) -> Result<Response<HelloServerStreamStream>, Status> {
        // mpsc (multi-producer, single-consumer) チャンネルの作成
        // サーバーはこのチャンネルにデータ送信し、その後クライアントにウトリーミングする
        // バッファサイズは適宜調整が必要だが、サーバーストリーミング機能なら1などでOK
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(async move {
            logger::info(&ctx, "Start Server Stream !!");
            // n件のデータをクライアントに返す（今回は3件）
            for i in 1..=3 {
                // レスポンスの設定
                let msg = format!("[{}] Hello {} !", i, req_body.text);
                let res_body = sample_proto::HelloServerStreamResponseBody {
                    message: msg,
                };

                // Okでラップしたレスポンスをtxで送信
                if let Err(e) = tx.send(Ok(res_body)).await {
                    // クライアントの接続切れなどでエラーの場合
                    let msg = format!("Failed to send data: {:?}", e);
                    logger::error(&ctx, &msg);
                    break;
                }

                // 1秒間待機処理
                sleep(Duration::from_secs(1)).await;
            }
            logger::info(&ctx, "Finish Server Stream !!");
        });

        // 戻り値
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
