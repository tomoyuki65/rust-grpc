use chrono::TimeZone;
use log;
use std::io::Write;

// 共通コンテキスト
use crate::contexts::context::Context;

// ロガーの初期化用関数
pub fn init_logger() {
    // 日本時間を取得
    let jst = chrono::offset::FixedOffset::east_opt(9 * 3600)
        .unwrap()
        .from_utc_datetime(&chrono::Utc::now().naive_utc());

    // カスタムロガーの初期化
    env_logger::builder()
        .format(move |buf, record| {
            writeln!(
                buf,
                "{} {} {}",
                jst.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

// 共通コンテキストからログに追加する情報の文字列を取得する関数
fn get_info_from_request(ctx: &Context) -> String {
    format!("request_id={} uri={}", ctx.request_id, ctx.uri)
}

// ログ出力用関数
pub fn info(ctx: &Context, msg: &str) {
    let info = get_info_from_request(ctx);
    log::info!("[{}] {}", info, msg);
}

pub fn warn(ctx: &Context, msg: &str) {
    let info = get_info_from_request(ctx);
    log::warn!("[{}] {}", info, msg);
}

// TODO: 使用する場合にコメントアウトを外す
// pub fn error(ctx: &Context, msg: &str) {
//     let info = get_info_from_request(ctx);
//     log::error!("[{}] {}", info, msg);
// }
