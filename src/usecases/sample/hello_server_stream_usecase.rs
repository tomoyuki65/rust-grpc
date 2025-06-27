// tonic
use tonic::{
    Response, Status,
    metadata::{Ascii, MetadataValue},
};

// tokio
use tokio::{
    spawn,
    sync::mpsc,
    time::{Duration, sleep},
};
use tokio_stream::wrappers::ReceiverStream;

// 変換用
use std::str::FromStr;

// 共通コンテキスト
use crate::contexts::context::Context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 実行するユースケースの構造体
pub struct SampleHelloServerStreamUsecase {}

// ストリーミング用の型定義
type HelloServerStreamStream =
    ReceiverStream<Result<sample_proto::HelloServerStreamResponseBody, Status>>;

impl SampleHelloServerStreamUsecase {
    pub async fn exec(
        &self,
        ctx: Context,
        req_body: sample_proto::HelloServerStreamRequestBody,
    ) -> Result<Response<HelloServerStreamStream>, Status> {
        // トレーラー用
        let x_request_id = MetadataValue::<Ascii>::from_str(ctx.request_id.as_str()).expect("-");

        // mpsc (multi-producer, single-consumer) チャンネルの作成
        // サーバーはこのチャンネルにデータ送信し、その後クライアントにストリーミングする
        // バッファサイズは適宜調整が必要だが、サーバーストリーミング機能なら1などでOK
        let (tx, rx) = mpsc::channel(1);

        spawn(async move {
            logger::info(&ctx, "Start Server Stream !!");
            // n件のデータをクライアントに返す（今回は3件）
            for i in 1..=3 {
                // レスポンスの設定
                let msg = format!("[{}] Hello {} !", i, req_body.text);
                let res_body = sample_proto::HelloServerStreamResponseBody { message: msg };

                // Okでラップしたレスポンスをtxで送信
                if let Err(e) = tx.send(Ok(res_body)).await {
                    // クライアントの接続切れなどでエラーの場合
                    let msg = format!("Failed to send data: {:?}", e);
                    logger::error(&ctx, &msg);
                    let mut status = Status::invalid_argument(msg);
                    status
                        .metadata_mut()
                        .insert("x-request-id", x_request_id.clone());
                    let _ = tx.send(Err(status)).await;
                    break;
                }

                // 1秒間待機処理
                sleep(Duration::from_secs(1)).await;
            }

            // トレーラーの設定
            let mut status = Status::ok("Stream finished successfully");
            status
                .metadata_mut()
                .insert("x-request-id", x_request_id.clone());

            // Errでラップしたステータスを送信
            if let Err(e) = tx.send(Err(status)).await {
                // クライアントの接続切れなどでエラーの場合
                let msg = format!("Failed to send data: {:?}", e);
                logger::error(&ctx, &msg);
                let mut status = Status::invalid_argument(msg);
                status.metadata_mut().insert("x-request-id", x_request_id);
                let _ = tx.send(Err(status)).await;
            }

            logger::info(&ctx, "Finish Server Stream !!");
        });

        // 戻り値
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
