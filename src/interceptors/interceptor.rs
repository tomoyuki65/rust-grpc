use tonic::{Code, Request, Status};

#[allow(clippy::result_large_err)]
pub fn auth_interceptor(req: tonic::Request<()>) -> Result<Request<()>, Status> {
    // メタデータからx-uriを取得
    let uri = match req.metadata().get("x-uri") {
        Some(value) => value.to_str().unwrap_or_default(),
        None => "-",
    };

    // 対象のuriの場合はスキップ
    let skip_uri = vec![
        "/sample.SampleService/Hello",
    ];
    if skip_uri.contains(&uri) {
        return Ok(req);
    }

    // リクエストヘッダーからトークン取得
    let token = match req.metadata().get("authorization") {
        Some(value) => {
            let bearer_token = value.to_str().unwrap_or_default();
            bearer_token.trim_start_matches("Bearer ")
        }
        None => "",
    };

    // 認証トークンが設定されていない場合はエラー
    if token.is_empty() {
        let status = Status::new(
            Code::InvalidArgument,
            "認証用トークンが設定されていません。",
        );
        return Err(status);
    }

    // TODO: 認証チェック処理を追加

    // 戻り値
    Ok(req)
}
