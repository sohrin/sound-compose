/*
 * TODO: extern crate（あっているか調査が必要）
 *       useのようなものだが、Rust 2018に対応していないクレートではextern crateする必要がある。
 *       マクロを導入する場合は#[macro_use]を付与する必要がある。
 *       （Rust 2018対応済クレートではマクロの名前解決で利用できるようになっているそう）
 *       https://qnighy.hatenablog.com/entry/2019/05/06/190000
 */
extern crate pretty_env_logger;
#[macro_use] extern crate log;

/*
 * MEMO: use文
 *       Javaのimport文。"."ではなく"::"で区切る。また、"{}"で複数指定可能。
 *       useしておけば、プロジェクト内や依存関係に追加したモジュールを利用時に
 *       絶対パスで指定する必要がなくなる。
 *       https://doc.rust-jp.rs/book/second-edition/ch02-00-guessing-game-tutorial.html#a%E4%BA%88%E6%83%B3%E3%82%92%E5%87%A6%E7%90%86%E3%81%99%E3%82%8B
 */
use std::env;
/*
 * MEMO: ライブラリ（を参照）で定義したモジュールのuseについて
 *       main.rsからmod.rs側で定義したライブラリのモジュールを利用する場合、
 *       use文のモジュール指定はCargo.tomlの[lib]のname値がrootになる。
 */
use sound_compose_lib::internal::apps::cli::common::args;

/*
 * MEMO: mainメソッド
 *       他の言語と同様、初めに実行される関数という認識でよい。
 *       main.rsに定義し、生成されたバイナリの起動や、cargo runコマンドで実行される。
 *       ライブラリクレートはlib.rsがルートのモジュールだが、main関数は不要。
 *       https://www.finddevguides.com/s/rust/rust_modules
 */
fn main() {
    /*
     * MEMO: ログ出力(初期処理)
     *       init後、println!マクロと同じ用法でログレベル!マクロを実行することでログが出力できる。
     *       main.rs＆lib.rsでpretty_env_loggerとlog(macro_use)をextern crateし、
     *       mainメソッドの冒頭でinitしておけば、他モジュールでのextern crateやinitは不要。
     *       init前に環境変数RUST_LOGを設定しておかないと動作しないことに注意。
     *       https://crates.io/crates/pretty_env_logger
     */
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();

    debug!("sound-compose running...");

    // 引数処理
    /*
     * TODO: 関数/メソッドの呼び出し（あっているか調査が必要、特に黒ポチ5つ目）
     *       モジュールに定義されたメソッドが「関数」、構造体にimplされたメソッドが「メソッド」という区別。
     *       ・関数呼び出し：モジュール::関数名(引数)
     *       ・同一モジュール内の関数呼び出し：関数名(引数)
     *       ・構造体のスタティックメソッド呼び出し：構造体名::メソッド名(引数)
     *       ・構造体のインスタンスメソッド（第一引数が&self）呼び出し：構造体インスタンス格納変数.メソッド名(引数、&selfに該当する引数は渡す必要なし)
     *       ・インスタンスメソッドから自身の別インスタンスメソッド呼び出し：self.メソッド名(引数、&selfに該当する引数は渡す必要なし)
     *       https://doc.rust-jp.rs/book/second-edition/ch03-03-how-functions-work.html
     *       https://doc.rust-jp.rs/book/second-edition/ch05-03-method-syntax.html
     */
    args::args_proc();

    println!("sound-compose finish!");
}