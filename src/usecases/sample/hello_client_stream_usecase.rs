// tonic
use tonic::{Response, Status, Request, Streaming};

// バリデーション用のトレイト
use prost_validate::Validator;

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

        // リクエストからストリームを取り出す
        let mut stream = request.into_inner();

        // ストリームからデータを1件ずつ受信し、ストリームが閉じるまでループ
        let mut lists = Vec::new();
        while let Some(req) = stream.message().await? {
            // バリデーションチェック
            match req.validate() {
                Ok(_) => {}
                Err(e) => {
                    let msg = format!("バリデーションエラー: {}", e);
                    logger::error(&ctx, &msg);
                    return Err(Status::invalid_argument(msg));
                }
            }

            // データを配列に格納
            lists.push(req.text);
        }

        // 配列のデータから文字列を作成
        let msg = lists.join(",");

        // レスポンスを設定
        let res = sample_proto::HelloClientStreamResponseBody {
            message: msg
        };

        // 戻り値
        Ok(Response::new(res))
    }
}
