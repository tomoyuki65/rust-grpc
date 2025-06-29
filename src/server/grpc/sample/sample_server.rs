// tonic
use tonic::{Request, Response, Status, Streaming};

// tokio
use tokio_stream::wrappers::ReceiverStream;

// バリデーション用のトレイト
use prost_validate::Validator;

// 共通コンテキスト
use crate::contexts::context::create_context;

// ロガー
use crate::loggers::logger;

// ビルドしたprotoファイルのインポート
pub mod sample_proto {
    // protoファイルに定義したpakage名を指定
    tonic::include_proto!("sample");
}

// リポジトリ
use crate::repositories::sample::sample_repository::SampleRepository;

// サービス
use crate::services::sample::sample_service::{SampleCommonRepository, SampleService};

// ユースケース
use crate::usecases::sample::hello_add_text_usecase::SampleHelloAddTextUsecase;
use crate::usecases::sample::hello_bidirectional_stream_usecase::SampleHelloBidirectionalStreamUsecase;
use crate::usecases::sample::hello_client_stream_usecase::SampleHelloClientStreamUsecase;
use crate::usecases::sample::hello_server_stream_usecase::SampleHelloServerStreamUsecase;
use crate::usecases::sample::hello_usecase::{SampleCommonService, SampleHelloUsecase};

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
        // コンテキスト設定
        let ctx = create_context(&request);

        // インスタンス化
        let sample_repo = Box::new(SampleRepository::new());
        let sample_common_repo = SampleCommonRepository { sample_repo };
        let sample_service = SampleService::new(sample_common_repo);
        let sample_common_service = SampleCommonService { sample_service };
        let usecase = SampleHelloUsecase {
            service: sample_common_service,
        };

        // ユースケースの実行
        usecase.exec(ctx).await
    }

    async fn hello_add_text(
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
                logger::error(&ctx, &msg);
                return Err(Status::invalid_argument(msg));
            }
        }

        // インスタンス化
        let usecase = SampleHelloAddTextUsecase {};

        // ユースケースの実行
        usecase.exec(ctx, req_body).await
    }

    // サーバーストリーミングの追加（typeの定義必須）
    // ※protoファイルで定義した名称+Streamという型の定義が必要になる
    type HelloServerStreamStream =
        ReceiverStream<Result<sample_proto::HelloServerStreamResponseBody, Status>>;

    async fn hello_server_stream(
        &self,
        request: Request<sample_proto::HelloServerStreamRequestBody>,
    ) -> Result<Response<Self::HelloServerStreamStream>, Status> {
        // コンテキスト設定
        let ctx = create_context(&request);

        // リクエストボディを取得
        let req_body = request.into_inner();

        // バリデーションチェック
        match req_body.validate() {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("バリデーションエラー: {}", e);
                logger::error(&ctx, &msg);
                return Err(Status::invalid_argument(msg));
            }
        }

        // インスタンス化
        let usecase = SampleHelloServerStreamUsecase {};

        // ユースケースの実行
        usecase.exec(ctx, req_body).await
    }

    // クライアントストリーミングの追加
    async fn hello_client_stream(
        &self,
        request: Request<Streaming<sample_proto::HelloClientStreamRequestBody>>,
    ) -> Result<Response<sample_proto::HelloClientStreamResponseBody>, Status> {
        // インスタンス化
        let usecase = SampleHelloClientStreamUsecase {};

        // ユースケースの実行
        usecase.exec(request).await
    }

    // 双方向ストリーミングの追加（typeの定義必須）
    // ※protoファイルで定義した名称+Streamという型の定義が必要になる
    type HelloBidirectionalStreamStream =
        ReceiverStream<Result<sample_proto::HelloBidirectionalStreamResponseBody, Status>>;

    async fn hello_bidirectional_stream(
        &self,
        request: Request<Streaming<sample_proto::HelloBidirectionalStreamRequestBody>>,
    ) -> Result<Response<Self::HelloBidirectionalStreamStream>, Status> {
        // インスタンス化
        let usecase = SampleHelloBidirectionalStreamUsecase {};

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
