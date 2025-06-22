use tonic::Request;

// // 共通コンテキストの構造体
#[derive(Clone, Debug)]
pub struct Context {
    #[allow(dead_code)]
    pub request_id: String,
    #[allow(dead_code)]
    pub uri: String,
}

// // コンテキスト作成関数
pub fn create_context<T>(req: &Request<T>) -> Context {
    let request_id = req
        .metadata()
        .get("x-request-id")
        .map(|value| value.to_str().unwrap_or("-"))
        .unwrap_or("-");

    let uri = req
        .metadata()
        .get("x-uri")
        .map(|value| value.to_str().unwrap_or("-"))
        .unwrap_or("-");

    Context {
        request_id: request_id.to_string(),
        uri: uri.to_string(),
    }
}
