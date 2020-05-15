use std::fmt;

// TODO: タプル構造体について（引数に見えるがタプルのフィールド定義なので、pubがないとコンストラクタがprivateと扱われる。）
pub struct YamlFileNotFoundError(pub std::io::Error);

// TODO: help: convert the identifier to snake case: `app_error`を対応する
//       でもEnumはCamelCaseでよさそうだが・・・？
//       エラー発生箇所「pub mod AppError」が問題か？
//       https://sinkuu.github.io/api-guidelines/naming.html
//       https://qiita.com/Scstechr/items/9b6d0d5461eb23adc28f
#[derive(Debug)]
pub enum AppError {
    YamlFileNotFoundError(std::io::Error),
}

// TODO: https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199#thiserror
// TODO: fmt::Display
// TODO: impl
impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::YamlFileNotFoundError(err) => write!(fmt, "{}", err),
        }
    }
}

// TODO: fmt::Displayをimplしないと動かないのが何故か理解する。
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::YamlFileNotFoundError(err) => err.source(),
        }
    }
}