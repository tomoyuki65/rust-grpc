// tonic
use tonic::{
    Request, Response, Status, Streaming,
    metadata::{Ascii, MetadataValue},
};

// バリデーション用のトレイト
use prost_validate::Validator;

// 変換用
use std::str::FromStr;

// 共通コンテキスト
use crate::contexts::context::create_context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 実行するユースケースの構造体
pub struct SampleHelloClientStreamUsecase {}

impl SampleHelloClientStreamUsecase {
    pub async fn exec(
        &self,
        request: Request<Streaming<sample_proto::HelloClientStreamRequestBody>>,
    ) -> Result<Response<sample_proto::HelloClientStreamResponseBody>, Status> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // トレーラー用
        let x_request_id = MetadataValue::<Ascii>::from_str(ctx.request_id.as_str()).expect("-");

        // リクエストからストリームを取り出す
        let mut stream = request.into_inner();

        logger::info(&ctx, "Start Client Stream !!");

        // ストリームからデータを1件ずつ受信し、ストリームが閉じるまでループ
        let mut lists = Vec::new();
        while let Some(req) = stream.message().await? {
            // バリデーションチェック
            match req.validate() {
                Ok(_) => {}
                Err(e) => {
                    let msg = format!("バリデーションエラー: {}", e);
                    logger::error(&ctx, &msg);
                    let mut status = Status::invalid_argument(msg);
                    status
                        .metadata_mut()
                        .insert("x-request-id", x_request_id.clone());
                    return Err(status);
                }
            }

            // データを配列に格納
            lists.push(req.text);
        }

        // 配列のデータから文字列を作成
        let msg = lists.join(",");

        // レスポンスを設定
        let mut res = Response::new(sample_proto::HelloClientStreamResponseBody { message: msg });

        // トレーラー設定
        res.metadata_mut().insert("x-request-id", x_request_id);

        logger::info(&ctx, "Finish Client Stream !!");

        // 戻り値
        Ok(res)
    }
}
