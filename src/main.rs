/* MEMO: use文
 *       Javaのimport文。
 *       https://doc.rust-jp.rs/book/second-edition/ch02-00-guessing-game-tutorial.html
 */
use std::env;

/* MEMO: mainメソッド
 *       他のcargo runコマンドで実行される。
 *       なお、バイナリクレートの場合はmain.rs、ライブラリクレートの場合はlib.rsを利用する。
 *       https://www.finddevguides.com/s/rust/rust_modules
 */
fn main() {
    println!("Hello, world!");

    /* MEMO: コマンドライン引数
     *       1要素目はexeファイル自体であることに注意。
     */
    /* MEMO: Vec
     *       Vec（ベクタ）は可変長配列。JavaのList。
     *       https://doc.rust-jp.rs/book/second-edition/ch08-01-vectors.html
     */
    let args: Vec<String> = env::args().collect();
    
    /* MEMO: println!マクロ
     *       標準出力マクロ。JavaのSystem.out.printlnメソッド。
     *       C言語のprintf関数のようにフォーマットが可能。
     *       フォーマットされた文字列だけがほしい場合ｈformat!マクロを使用する。
     *       {:?}は構造体の詳細がわかるようなフォーマット（fmt::Debugの実装）を行う。
     *       https://qiita.com/YusukeHosonuma/items/13142ab1518ccab425f4
     */
    println!("{:?}", args);

    // 
}
