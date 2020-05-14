use std::env;
use structopt::{clap, StructOpt};

/*
 * MEMO: StructOpt(引数処理ライブラリ)
 *       https://docs.rs/crate/structopt/0.3.14
 */
// docker-compose コマンド概要(https://docs.docker.com/compose/reference/overview/)を参考に
/*
 * MEMO: derive(継承)
 *       Javaのextendのようなものだが、複数指定可能。
 *       デフォルト実装を持ちオーバーライドも可能なマーカインタフェースっぽい感じ。
 *       https://doc.rust-jp.rs/rust-by-example-ja/trait/derive.html
 *       https://qiita.com/apollo_program/items/2495dda519ae160971ed
 */
#[derive(Debug, StructOpt)]
// TODO: structoptの説明
#[structopt(name = "sound-compose")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(
        short = "f",
        long,
        help = "Specify an alternate sound-compose file.\n(default: sound-compose.yml)")]
    file: Option<String>,

    #[structopt(
        long,
        help = "Show more output")]
    verbose: bool,

    #[structopt(subcommand)]
    sub: Sub,
}

#[derive(Debug, StructOpt)]
pub enum Sub {
    #[structopt(
        name = "build",
        about = "Exec build(bounce)."
    )]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Build (BuildOpts),
}

#[derive(Debug, StructOpt)]
pub struct BuildOpts {
    #[structopt(
        long = "no-cache",
        help = "Do not use cache when building."
    )]
    no_cache: bool,
}

// TODO: rustdocをしっかり書く
/*
 * MEMO: rustdpcドキュメンテーションコメント
 *       直前で"///"を用いるか、囲われたブロックの冒頭で"//!"を用いる。
 *       cargo doc --no-depsを実行すれば、本クレートのみのrustdoc実行が行われる。
 *       http://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/documentation.html
 */
/// 引数処理
pub fn args_proc() {
    /*
     * MEMO: コマンドライン引数
     *       1要素目はexeファイル自体であることに注意。
     *       https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c
     */
    /*
     * MEMO: Vec
     *       Vec（ベクタ）は可変長配列。JavaのList。
     *       https://doc.rust-jp.rs/book/second-edition/ch08-01-vectors.html
     */
    /*
     * MEMO: println!マクロ
     *       標準出力マクロ。JavaのSystem.out.printlnメソッド。
     *       C言語のprintf関数のようにフォーマットが可能。
     *       フォーマットされた文字列だけがほしい場合はformat!マクロを使用する。
     *       {:?}は構造体の詳細がわかるようなフォーマット（fmt::Debugの実装）を行う。
     *       https://qiita.com/YusukeHosonuma/items/13142ab1518ccab425f4
     */
    /* 
     * MEMO: 変数宣言
     *       型指定しない場合は推測してくれる。明示的に指定した場合は==の比較が厳密になる。
     *       https://qiita.com/aimof/items/c00b911ef5fb8cfed149
     */
    // コマンドライン引数はStructOptで受け取るためログ出力のみ実施
    let args: Vec<String> = std::env::args().collect();
    debug!("{:?}", args);

    // StructOptで受け取った引数情報
    let mut opt = Opt::from_args();
    debug!("引数チェック前 {:?}", opt);
    /*
     * MEMO: if文
     *       条件式の"("と")"が不要である点以外は他の言語と変わらない。
     */
    if opt.verbose == true {
        // ログレベル
        env::set_var("RUST_LOG", "debug");
    }
    if opt.file.is_none() {
        // TODO: https://www.ameyalokare.com/rust/2017/10/23/rust-options.html
        //       https://doc.rust-lang.org/std/option/enum.Option.html
        //       https://stackoverflow.com/questions/50673567/how-to-use-an-enum-that-represents-subcommands-with-structopt
        //       https://qiita.com/dalance/items/56dba0dd54c82f937feb
        opt.file = Some(String::from("sound-compose.yml"));
    }
    info!("引数チェック後 {:?}", opt);
    println!("引数チェック後 {:?}", opt);

    // yamlファイルを読み込む
}