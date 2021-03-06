use std::fmt;

use std::fmt::Display;

/* TODO: タプル構造体について
 *       フィールドのない構造体、カンマ区切りで指定
 *       参照は配列のように0から始まるインデックス（ex: 構造体変数名.0）
 *       引数に見えるがタプルのフィールド定義なので、pubがないとコンストラクタがprivateと扱われる。
 *       newtype patternでも利用する。
 *       https://keens.github.io/blog/2018/12/15/rustdetsuyomenikatawotsukerupart_1__new_type_pattern/
 */
pub struct YamlFileNotFoundError(pub std::io::Error);

// TODO: help: convert the identifier to snake case: `app_error`を対応する
//       でもEnumはCamelCaseでよさそうだが・・・？
//       エラー発生箇所「pub mod AppError」が問題か？
//       https://sinkuu.github.io/api-guidelines/naming.html
//       https://qiita.com/Scstechr/items/9b6d0d5461eb23adc28f
#[derive(Debug)]
pub enum AppError {
    YamlFileNotFoundError(std::io::Error),
    YamlDeserializeError(serde_yaml::Error),
}

// TODO: https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199#thiserror
// TODO: fmt::Display
// TODO: impl
impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::YamlFileNotFoundError(err) => write!(fmt, "{}", err),
            Self::YamlDeserializeError(err) => write!(fmt, "{}", err),
        }
    }
}

// TODO: fmt::Displayをimplしないと動かないのが何故か理解する。
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::YamlFileNotFoundError(err) => err.source(),
            Self::YamlDeserializeError(err) => err.source(),
        }
    }
}


// TODO: 後で改善したほうが良い
// https://doc.rust-lang.org/std/io/struct.Error.html
#[derive(Debug)]
pub struct MyError {
    v: String,
}

impl MyError {
    pub fn new() -> MyError {
        MyError {
            v: "oh no!".to_string()
        }
    }

    fn change_message(&mut self, new_message: &str) {
        self.v = new_message.to_string();
    }
}

impl std::error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", &self.v)
    }
}