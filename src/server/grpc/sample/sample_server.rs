// tonic
use tonic::{Request, Response, Status};

// ビルドしたprotoファイルのインポート
pub mod sample_proto {
    // protoファイルに定義したpakage名を指定
    tonic::include_proto!("sample");
}

// ユースケース
use crate::usecases::sample::hello_add_text_usecase::SampleHelloAddTextUsecase;
use crate::usecases::sample::hello_usecase::SampleHelloUsecase;

// 構造体定義
#[derive(Debug, Default)]
pub struct SampleServer {}

// protoファイルの関数の実装をメソッド定義
#[tonic::async_trait]
impl sample_proto::sample_service_server::SampleService for SampleServer {
    async fn hello(
        &self,
        request: Request<sample_proto::Empty>,
    ) -> Result<Response<sample_proto::HelloResponseBody>, Status> {
        // インスタンス化
        let usecase = SampleHelloUsecase {};

        // ユースケースの実行
        usecase.exec(request).await
    }

    async fn hello_add_text(
        &self,
        request: Request<sample_proto::HelloAddTextRequestBody>,
    ) -> Result<Response<sample_proto::HelloAddTextResponseBody>, Status> {
        // インスタンス化
        let usecase = SampleHelloAddTextUsecase {};

        // ユースケースの実行
        usecase.exec(request).await
    }
}

// ルーターに設定するサーバー定義を返す関数
pub fn get_sample_server() -> sample_proto::sample_service_server::SampleServiceServer<SampleServer>
{
    let sample_server = SampleServer::default();
    sample_proto::sample_service_server::SampleServiceServer::new(sample_server)
}
