use hyper::header::HeaderValue;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tonic::server::NamedService;
use tower::{Layer, Service};
use uuid::Uuid;

// 共通コンテキスト
use crate::contexts::context::Context as CommonContext;

// ロガー
use crate::loggers::logger;

// リクエスト用のミドルウェア
#[derive(Clone)]
pub struct RequestMiddleware<S> {
    inner: S,
}

impl<S> RequestMiddleware<S> {
    fn new(inner: S) -> Self {
        RequestMiddleware { inner }
    }
}

// リクエスト用のミドルウェアにServiceトレイトを実装
impl<B, S> Service<hyper::Request<B>> for RequestMiddleware<S>
where
    S: Service<hyper::Request<B>> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<B>) -> Self::Future {
        // リクエストからuriを取得
        let uri_path = req.uri().path();

        // uriをリクエストヘッダーに設定
        let uri_string = match HeaderValue::from_str(uri_path) {
            Ok(value) => {
                req.headers_mut().insert("x-uri", value.clone());
                value.to_str().unwrap().to_string()
            }
            Err(_) => {
                // エラーの場合は「-」を設定
                let header_value = HeaderValue::from_str("-").unwrap();
                req.headers_mut().insert("x-uri", header_value.clone());
                header_value.to_str().unwrap().to_string()
            }
        };

        // request-idをリクエストヘッダーに設定
        let uuid = Uuid::new_v4().to_string();
        req.headers_mut()
            .insert("x-request-id", uuid.clone().parse().unwrap());

        // リクエスト開始ログ
        let ctx = CommonContext {
            request_id: uuid,
            uri: uri_string,
        };
        logger::info(&ctx, "Start gRPC request !!");

        // 非同期処理のためself.innerをコピー
        let mut inner = self.inner.clone();

        // 非同期処理
        Box::pin(async move {
            let res = inner.call(req).await;

            // 処理完了後にリクエスト終了ログ
            logger::info(&ctx, "Finish gRPC request !!");

            res
        })
    }
}

// リクエスト用のミドルウェアにtonic::server::NamedServiceを実装
impl<S> NamedService for RequestMiddleware<S>
where
    S: NamedService,
{
    const NAME: &'static str = S::NAME;
}

// リクエスト用のミドルウェアのレイヤー
#[derive(Clone)]
pub struct RequestMiddlewareLayer;

// リクエスト用のミドルウェアのレイヤーにLayerトレイトの実装
impl<S> Layer<S> for RequestMiddlewareLayer {
    type Service = RequestMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestMiddleware::new(inner)
    }
}
