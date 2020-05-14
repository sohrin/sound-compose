/*
 * MEMO: ログ出力
 *       pretty_env_logger::init();を実行すれば、ログレベル!マクロが使えるようになる。
 *       https://crates.io/crates/pretty_env_logger
 */
// TODO: 言語仕様を調べる
extern crate pretty_env_logger;
// TODO: 言語仕様を調べる
#[macro_use]
extern crate log;
 
/*
 * MEMO: use文
 *       Javaのimport文。"."ではなく"::"で区切る。また、"{}"で複数指定可能
 *       https://doc.rust-jp.rs/book/second-edition/ch02-00-guessing-game-tutorial.html
 */
use std::env;
use sound_compose_lib::internal::apps::cli::common::args;

/*
 * MEMO: mainメソッド
 *       他のcargo runコマンドで実行される。
 *       なお、バイナリクレートの場合はmain.rs、ライブラリクレートの場合はlib.rsを利用する。
 *       https://www.finddevguides.com/s/rust/rust_modules
 */
fn main() {
    /*
     * MEMO: ログ出力(初期処理)
     */
    pretty_env_logger::init();

    // TODO: 後で消す
    // TODO: ログレベルをデバッグにして一度ログが出るようになったのに出なくなった・・・？？？
    //       https://qiita.com/Dsuke-K/items/163a312bdd2b8a260615
    env::set_var("RUST_LOG", "debug");
    debug!("sound-compose running...");
    println!("sound-compose running...");

    // 引数処理
    args::args_proc();

    println!("sound-compose finish!");
}