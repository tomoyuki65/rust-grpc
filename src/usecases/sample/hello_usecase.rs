use tonic::{Response, Status};

// 共通コンテキスト
use crate::contexts::context::Context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// サービスのモジュール
use crate::services::sample::sample_service::{SampleService, SampleServiceTrait};

// 使用するサービスをまとめる構造体
pub struct SampleCommonService {
    pub sample_service: SampleService,
}

// 実行するユースケースの構造体
pub struct SampleHelloUsecase {
    pub service: SampleCommonService,
}

impl SampleHelloUsecase {
    pub async fn exec(
        &self,
        ctx: Context,
    ) -> Result<Response<sample_proto::HelloResponseBody>, Status> {
        // サンプルテキストを取得するサービスを実行
        let text = match self
            .service
            .sample_service
            .sample_get_text_hello(&ctx)
            .await
        {
            Ok(text) => text,
            Err(e) => {
                let msg = format!("Internal Server Error: {}", e);
                logger::error(&ctx, &msg);
                return Err(Status::unknown(msg));
            }
        };

        // レスポンスボディの設定
        let res_body = sample_proto::HelloResponseBody { message: text };

        // メタデータにrequest-idを追加
        let mut res = Response::new(res_body);
        res.metadata_mut()
            .insert("x-request-id", ctx.request_id.parse().unwrap());

        // 戻り値
        Ok(res)
    }
}
