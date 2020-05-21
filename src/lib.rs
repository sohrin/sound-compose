// TODO: https://qiita.com/yasuyuky/items/45f7333118fa165b670b に関する説明
// TODO: 「#![crate_type = "dylib"]」の代わりに、Cargo.tomlに「crate-type = ["rlib", "cdylib"]
#[no_mangle]
pub extern fn repeat_hello(s:i32) {
    for _ in 0..s {
        println!("Hello");
    }
}

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

/*
 * MEMO: モジュール定義
 *       main.rsでもmodできるが、lib.rs側でmodし、main.rs側で使うようにする（共通部品として使える）。
 *       https://keens.github.io/blog/2017/01/15/rustnomoju_runokirikata/
 *       https://www.it-swarm.dev/ja/rust/1050664534/
 *       https://qiita.com/nirasan/items/8804046c43ba07ee8fde
 *       https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/crates-and-modules.html
 *       https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
 */
// TODO: pub useについて・・・https://qiita.com/dalance/items/917ef0d587d884ecc69f
pub mod internal;
pub mod common;