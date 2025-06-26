// tonic
use tonic::{Request, Response, Status};

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 共通コンテキスト
use crate::contexts::context::create_context;

// ロガー
use crate::loggers::logger;

// バリデーション用のトレイト
use prost_validate::Validator;

// 実行するユースケースの構造体
pub struct SampleHelloAddTextUsecase {}

impl SampleHelloAddTextUsecase {
    pub async fn exec(
        &self,
        request: Request<sample_proto::HelloAddTextRequestBody>,
    ) -> Result<Response<sample_proto::HelloAddTextResponseBody>, Status> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // リクエストボディを取得
        let req_body = request.into_inner();

        // バリデーションチェック
        match req_body.validate() {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("バリデーションエラー: {}", e);
                logger::warn(&ctx, &msg);
                return Err(Status::invalid_argument(msg));
            }
        };

        // レスポンスメッセージの設定
        let msg = format!("Hello {}", req_body.text);

        // レスポンスボディの設定
        let res_body = sample_proto::HelloAddTextResponseBody { message: msg };

        // メタデータにrequest-idを追加
        let mut res = Response::new(res_body);
        res.metadata_mut()
            .insert("x-request-id", ctx.request_id.parse().unwrap());

        // 戻り値
        Ok(res)
    }
}
