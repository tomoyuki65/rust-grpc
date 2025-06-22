use tonic::{Request, Response, Status};

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 共通コンテキスト
use crate::contexts::context::create_context;

// ロガー
// use crate::loggers::logger;

// 実行するユースケースの構造体
pub struct SampleHelloUsecase {}

impl SampleHelloUsecase {
    pub async fn exec(
        &self,
        request: Request<sample_proto::Empty>,
    ) -> Result<Response<sample_proto::HelloResponseBody>, Status> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // レスポンスボディの設定
        let res_body = sample_proto::HelloResponseBody {
            message: "Hello World !!".to_string(),
        };

        // メタデータにrequest-idを追加
        let mut res = Response::new(res_body);
        res.metadata_mut()
            .insert("x-request-id", ctx.request_id.parse().unwrap());

        // 戻り値
        Ok(res)
    }
}
