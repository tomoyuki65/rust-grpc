// tonic
use tonic::{
    Response,
    Status,
    Request,
    Streaming,
    metadata::{
        MetadataValue,
        Ascii,
    },
};

// tokio
use tokio::{
    sync::mpsc,
    spawn,
    time::{sleep, Duration},
};
use tokio_stream::{
    wrappers::ReceiverStream,
    StreamExt,
};

// 変換用
use std::str::FromStr;

// バリデーション用のトレイト
use prost_validate::Validator;

// 共通コンテキスト
use crate::contexts::context::create_context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 実行するユースケースの構造体
pub struct SampleHelloBidirectionalStreamUsecase {}

// ストリーミング用の型定義
type HelloBidirectionalStreamStream = ReceiverStream<Result<sample_proto::HelloBidirectionalStreamResponseBody, Status>>;

impl SampleHelloBidirectionalStreamUsecase {
    pub async fn exec(
        &self,
        request: Request<Streaming<sample_proto::HelloBidirectionalStreamRequestBody>>,
    ) -> Result<Response<HelloBidirectionalStreamStream>, Status,> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // トレーラー用
        let x_request_id = MetadataValue::<Ascii>::from_str(ctx.request_id.as_str()).expect("-");

        // リクエストからストリームを取り出す
        let mut stream = request.into_inner();

        // mpsc (multi-producer, single-consumer) チャンネルの作成
        // サーバーはこのチャンネルにデータ送信し、その後クライアントにストリーミングする
        // バッファサイズは適宜調整が必要
        let (tx, rx) = mpsc::channel(1);

        spawn(async move {
            logger::info(&ctx, "Start Bidirectional Stream !!");
            while let Some(result) = stream.next().await {
                match result {
                    Ok(req) => {
                        // バリデーションチェック
                        match req.validate() {
                            Ok(_) => {}
                            Err(e) => {
                                let msg = format!("バリデーションエラー: {}", e);
                                logger::error(&ctx, &msg);
                                let mut status = Status::invalid_argument(msg);
                                status.metadata_mut().insert("x-request-id", x_request_id.clone());
                                let _ = tx.send(Err(status)).await;
                                break;
                            }
                        }

                        // レスポンスを設定
                        let res = sample_proto::HelloBidirectionalStreamResponseBody {
                            message: req.text
                        };

                        // Okでラップしたレスポンスをtxで送信
                        if let Err(e) = tx.send(Ok(res)).await {
                            // クライアントの接続切れなどでエラーの場合
                            let msg = format!("Failed to send data: {:?}", e);
                            logger::error(&ctx, &msg);
                            let mut status = Status::invalid_argument(msg);
                            status.metadata_mut().insert("x-request-id", x_request_id.clone());
                            let _ = tx.send(Err(status)).await;
                            break;
                        }

                        // 1秒間待機処理
                        sleep(Duration::from_secs(1)).await;
                    }
                    Err(e) => {
                        // クライアントの接続切れなどでエラーの場合
                        let msg = format!("Failed to send data: {:?}", e);
                        logger::error(&ctx, &msg);
                        let mut status = Status::invalid_argument(msg);
                        status.metadata_mut().insert("x-request-id", x_request_id.clone());
                        let _ = tx.send(Err(status)).await;
                        break;
                    }
                }
            }

            // トレーラーの設定
            let mut status = Status::ok("Stream finished successfully");
            status.metadata_mut().insert("x-request-id", x_request_id.clone());

            // Errでラップしたステータスを送信
            if let Err(e) = tx.send(Err(status)).await {
                // クライアントの接続切れなどでエラーの場合
                let msg = format!("Failed to send data: {:?}", e);
                logger::error(&ctx, &msg);
                let mut status = Status::invalid_argument(msg);
                status.metadata_mut().insert("x-request-id", x_request_id);
                let _ = tx.send(Err(status)).await;
            }

            logger::info(&ctx, "Finish Bidirectional Stream !!");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
