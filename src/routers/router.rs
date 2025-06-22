use tonic::{service::InterceptorLayer, transport::Server};
use tonic_reflection::server::Builder as ReflectionBuilder;
use tower::ServiceBuilder;

// protoファイル
pub mod proto {
    pub const SAMPLE_DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("sample_descriptor");
}

// gRPCサーバーのサービス
use crate::server::grpc::sample::sample_server::get_sample_server;

// ミドルウェア
use crate::middleware::request_middleware::RequestMiddlewareLayer;

// インターセプター
use crate::interceptors::interceptor::auth_interceptor;

pub async fn router() -> Result<(), Box<dyn std::error::Error>> {
    // アドレス設定
    let grpc_port = std::env::var("GRPC_PORT").unwrap_or("50051".to_string());
    let addr = format!("[::]:{}", grpc_port).parse()?;

    // サーバー設定
    let sample_server = get_sample_server();

    // サーバーリフレクション設定（旧ツールではv1alphaを使う）
    let reflection_service_v1 = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::SAMPLE_DESCRIPTOR)
        .build_v1()?;

    let reflection_service_v1alpha = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::SAMPLE_DESCRIPTOR)
        .build_v1alpha()?;

    // サービス設定
    let service = ServiceBuilder::new()
        .layer(RequestMiddlewareLayer)
        .layer(InterceptorLayer::new(auth_interceptor))
        .service(sample_server);

    // サーバー起動
    Server::builder()
        .add_service(reflection_service_v1)
        .add_service(reflection_service_v1alpha)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
