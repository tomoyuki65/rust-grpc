#[cfg(test)]

// sample_serverのテスト
mod sample_server_test {
    use tonic::Request;
    use crate::server::grpc::sample::sample_server::{
        SampleServer,
        sample_proto::{
            sample_service_server::SampleService,
            sample_service_client::SampleServiceClient,
            Empty,
            HelloAddTextRequestBody,
        },
    };

    #[tokio::test]
    async fn hello_should_return_should_succeed() {
        // サーバーのインスタンス作成
        let server = SampleServer::default();
        let request = Request::new(Empty {});

        // テストの実行
        let response = server.hello(request).await;

        // 検証
        assert!(response.is_ok());
        let response_body = response.unwrap().into_inner();
        assert_eq!(response_body.message, "Hello World !!");
    }

    #[tokio::test]
    async fn hello_add_text_should_succeed() {
        // サーバーのインスタンス作成
        let port = "50051";
        let mut client = SampleServiceClient::connect(format!("http://localhost:{}", port)).await.unwrap();

        // リクエストの作成
        let mut request = Request::new(HelloAddTextRequestBody {
            text: "Add World !!".to_string(),
        });

        // メタデータにBearerトークンを追加
        request.metadata_mut().insert(
            "authorization",
            "Bearer token".parse().unwrap(),
        );

        // テストの実行
        let response = client.hello_add_text(request).await;

        // 検証
        assert!(response.is_ok());
        let response_body = response.unwrap().into_inner();
        assert_eq!(response_body.message, "Hello Add World !!");
    }

    #[tokio::test]
    async fn hello_add_text_should_fail_with_empty_text() {
        // サーバーのインスタンス作成
        let port = "50051";
        let mut client = SampleServiceClient::connect(format!("http://localhost:{}", port)).await.unwrap();

        // リクエストの作成
        let mut request = Request::new(HelloAddTextRequestBody {
            text: "".to_string(),
        });
        // インターセプターがチェックする無効な認証ヘッダーを追加
        request.metadata_mut().insert(
            "authorization",
            "Bearer token".parse().unwrap(),
        );

        let response = client.hello_add_text(request).await;

        // 検証
        assert!(response.is_err());
        let status = response.err().unwrap();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert!(status.message().contains("バリデーションエラー"));
    }
}