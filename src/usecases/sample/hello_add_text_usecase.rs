// tonic
use tonic::{Request, Response, Status};

// ビルドしたprotoファイルのモジュール
use crate::server::grpc::sample::sample_server::sample_proto;

// 共通コンテキスト
use crate::contexts::context::create_context;

// 実行するユースケースの構造体
pub struct SampleHelloAddTextUsecase {}

impl SampleHelloAddTextUsecase {
    pub async fn exec(
        &self,
        request: Request<sample_proto::HelloAddTextRequestBody>,
    ) -> Result<Response<sample_proto::HelloAddTextResponseBody>, Status> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // レスポンスボディの設定
        let res_body = sample_proto::HelloAddTextResponseBody {
            message: "hello world !!".to_string(),
        };

        // メタデータにrequest-idを追加
        let mut res = Response::new(res_body);
        res.metadata_mut()
            .insert("x-request-id", ctx.request_id.parse().unwrap());

        // 戻り値
        Ok(res)
    }
}
