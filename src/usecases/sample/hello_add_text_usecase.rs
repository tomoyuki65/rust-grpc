// tonic
use tonic::{Response, Status};

// 共通コンテキスト
use crate::contexts::context::Context;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 実行するユースケースの構造体
pub struct SampleHelloAddTextUsecase {}

impl SampleHelloAddTextUsecase {
    pub async fn exec(
        &self,
        ctx: Context,
        req_body: sample_proto::HelloAddTextRequestBody,
    ) -> Result<Response<sample_proto::HelloAddTextResponseBody>, Status> {
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
