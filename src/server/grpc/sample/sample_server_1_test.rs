#[cfg(test)]

// sample_serverのテスト
mod sample_server_test {
    use crate::configs::config::get_config;
    use crate::contexts::context::Context;
    use crate::server::grpc::sample::sample_server::sample_proto::{
        HelloAddTextRequestBody, sample_service_client::SampleServiceClient,
    };
    use tonic::Request;
    // use crate::repositories::sample::sample_repository::SampleRepository;
    use crate::repositories::sample::sample_repository::MockSampleRepositoryTrait;
    use crate::services::sample::sample_service::{SampleCommonRepository, SampleService};
    use crate::usecases::sample::hello_usecase::{SampleCommonService, SampleHelloUsecase};

    #[tokio::test]
    async fn hello_should_return_should_succeed() {
        /* ユースケースを実行して検証する場合 */

        // サンプルリポシトリーのインスタンス化
        // リポジトリーのモック化が必要な場合
        let mut mock_repo = MockSampleRepositoryTrait::new();
        mock_repo
            .expect_sample_hello()
            .returning(|_| Ok("Mock Hello World !!".to_string()));
        let sample_repo = Box::new(mock_repo);

        // インスタンス化
        let sample_common_repo = SampleCommonRepository { sample_repo };
        let sample_service = SampleService::new(sample_common_repo);
        let sample_common_service = SampleCommonService { sample_service };
        let usecase = SampleHelloUsecase {
            service: sample_common_service,
        };

        // コンテキストの設定
        let ctx = Context {
            request_id: "5ccba39d-fc9e-482a-aa6b-b94a450a53d0".to_string(),
            uri: "/sample.SampleService/Hello".to_string(),
        };

        // ユースケースの実行
        let res = usecase.exec(ctx).await;

        // 検証
        assert!(res.is_ok());
        let res_body = res.unwrap().into_inner();
        assert_eq!(res_body.message, "Mock Hello World !!");
    }

    #[tokio::test]
    async fn hello_add_text_should_succeed() {
        let config = get_config();

        // サーバーのインスタンス作成
        let mut client =
            SampleServiceClient::connect(format!("http://localhost:{}", config.grpc_port))
                .await
                .unwrap();

        // リクエストの作成
        let mut request = Request::new(HelloAddTextRequestBody {
            text: "Add World !!".to_string(),
        });

        // メタデータにBearerトークンを追加
        request
            .metadata_mut()
            .insert("authorization", "Bearer token".parse().unwrap());

        // テストの実行
        let response = client.hello_add_text(request).await;

        // 検証
        assert!(response.is_ok());
        let response_body = response.unwrap().into_inner();
        assert_eq!(response_body.message, "Hello Add World !!");
    }

    #[tokio::test]
    async fn hello_add_text_should_fail_with_empty_text() {
        let config = get_config();

        // サーバーのインスタンス作成
        let mut client =
            SampleServiceClient::connect(format!("http://localhost:{}", config.grpc_port))
                .await
                .unwrap();

        // リクエストの作成
        let mut request = Request::new(HelloAddTextRequestBody {
            text: "".to_string(),
        });
        // インターセプターがチェックする無効な認証ヘッダーを追加
        request
            .metadata_mut()
            .insert("authorization", "Bearer token".parse().unwrap());

        let response = client.hello_add_text(request).await;

        // 検証
        assert!(response.is_err());
        let status = response.err().unwrap();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert!(status.message().contains("バリデーションエラー"));
    }
}
