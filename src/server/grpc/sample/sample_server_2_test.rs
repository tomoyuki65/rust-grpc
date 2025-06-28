#[cfg(test)]

// sample_serverのストリーミングのテスト
mod sample_server_streaming_test {
    use tokio_stream::{StreamExt, iter};
    use tonic::Request;
    use crate::configs::config::get_config;
    use crate::contexts::context::Context;
    use crate::server::grpc::sample::sample_server::sample_proto::{
        HelloServerStreamRequestBody,
        HelloClientStreamRequestBody,
        HelloBidirectionalStreamRequestBody,
        sample_service_client::SampleServiceClient,
    };
    use crate::usecases::sample::hello_server_stream_usecase::SampleHelloServerStreamUsecase;

    #[tokio::test]
    async fn hello_server_stream_should_return_succeed_by_usecase() {
        /* ユースケースを実行して検証する場合 */

        // インスタンス化
        let usecase = SampleHelloServerStreamUsecase {};

        // コンテキストの設定
        let ctx = Context {
            request_id: "5ccba39d-fc9e-482a-aa6b-b94a450a53d0".to_string(),
            uri: "/sample.SampleService/HelloServerStream".to_string(),
        };

        // リクエストの設定
        let req = HelloServerStreamRequestBody {
            text: "Server Stream".to_string(),
        };

        // ユースケースの実行
        let res = usecase.exec(ctx, req).await;

        // 検証
        assert!(res.is_ok());

        // ストリームの検証
        let mut stream = res.unwrap().into_inner();
        let mut i = 1;
        while let Some(result) = stream.next().await {
            match result {
                Ok(res) => {
                    let msg = format!("[{}] Hello Server Stream !", i);
                    assert_eq!(res.message, msg);
                    
                }
                Err(_) => {}
            }
            i += 1;
        }
    }

    #[tokio::test]
    async fn hello_server_stream_should_return_succeed_by_client() {
        let config = get_config();

        // サーバーのインスタンス作成
        let mut client =
            SampleServiceClient::connect(format!("http://localhost:{}", config.grpc_port))
                .await
                .unwrap();

        // リクエストの作成
        let mut request = Request::new(HelloServerStreamRequestBody {
            text: "Server Stream".to_string(),
        });

        // メタデータにBearerトークンを追加
        request
            .metadata_mut()
            .insert("authorization", "Bearer token".parse().unwrap());

        // テストの実行
        let res = client.hello_server_stream(request).await;

        // 検証
        assert!(res.is_ok());

        // ストリームの検証
        let mut stream = res.unwrap().into_inner();
        let mut i = 1;
        while let Some(result) = stream.next().await {
            match result {
                Ok(res) => {
                    let msg = format!("[{}] Hello Server Stream !", i);
                    assert_eq!(res.message, msg);
                    
                }
                Err(_) => {}
            }
            i += 1;
        }
    }

    #[tokio::test]
    async fn hello_client_stream_should_return_succeed() {
        let config = get_config();

        // サーバーのインスタンス作成
        let mut client =
            SampleServiceClient::connect(format!("http://localhost:{}", config.grpc_port))
                .await
                .unwrap();

        // 送信するメッセージリスト作成
        let message_lists = vec![
            HelloClientStreamRequestBody { text: "A".to_string() },
            HelloClientStreamRequestBody { text: "B".to_string() },
            HelloClientStreamRequestBody { text: "C".to_string() },
        ];

        // ストリームの作成
        let mut request = Request::new(iter(message_lists));
        
        // メタデータにBearerトークンを追加
        request
            .metadata_mut()
            .insert("authorization", "Bearer token".parse().unwrap());

        // テストの実行
        let res = client.hello_client_stream(request).await;

        // 検証
        assert!(res.is_ok());
        let res_body = res.unwrap().into_inner();
        assert_eq!(res_body.message, "A,B,C");
    }

    #[tokio::test]
    async fn hello_bidirectional_stream_should_return_succeed() {
        let config = get_config();

        // サーバーのインスタンス作成
        let mut client =
            SampleServiceClient::connect(format!("http://localhost:{}", config.grpc_port))
                .await
                .unwrap();

        // 送信するメッセージリスト作成
        let message_lists = vec![
            HelloBidirectionalStreamRequestBody { text: "A".to_string() },
            HelloBidirectionalStreamRequestBody { text: "B".to_string() },
        ];

        // ストリームの作成
        let mut request = Request::new(iter(message_lists.clone()));
        
        // メタデータにBearerトークンを追加
        request
            .metadata_mut()
            .insert("authorization", "Bearer token".parse().unwrap());

        // テストの実行
        let res = client.hello_bidirectional_stream(request).await;

        // 検証
        assert!(res.is_ok());

        // ストリームの検証
        let mut stream = res.unwrap().into_inner();
        let mut i = 0;
        while let Some(result) = stream.next().await {
            match result {
                Ok(res) => {
                    let msg = message_lists[i].text.clone();
                    assert_eq!(res.message, msg);
                    
                }
                Err(_) => {}
            }
            i += 1;
        }
    }
}
