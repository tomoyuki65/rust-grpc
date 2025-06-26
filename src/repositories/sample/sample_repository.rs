// 共通コンテキスト
use crate::contexts::context::Context;

// コンフィング設定
use crate::configs::config::get_config;

// ロガー
use crate::loggers::logger;

// 共通エラー用モジュール
use crate::errors::error::CommonError;

// サンプルリポジトリーの構造体
pub struct SampleRepository;

impl SampleRepository {
    // 初期化用メソッド
    pub fn new() -> Self {
        SampleRepository
    }
}

// サンプルリポジトリー用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait SampleRepositoryTrait {
    async fn sample_hello(&self, ctx: &Context) -> Result<String, CommonError>;
}

#[async_trait::async_trait]
impl SampleRepositoryTrait for SampleRepository {
    // 文字列「Sample Hello !!」を返す関数
    async fn sample_hello(&self, ctx: &Context) -> Result<String, CommonError> {
        let mut text = "Hello World !!".to_string();

        let config = get_config();
        if config.env == "testing" {
            text = "Hello World !! [ENV=testing]".to_string();
        }

        if text.is_empty() {
            logger::error(ctx, "textが空です");
            return Err(CommonError::InternalServerError);
        }

        Ok(text)
    }
}
