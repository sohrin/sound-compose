// extern crate pretty_env_logger;
// #[macro_use]
// extern crate log;

use std::env;
use std::fs::File;
use structopt::{clap, StructOpt};

use crate::common::AppError;
use crate::internal::apps::cli::common::yaml;
// TODO: なぜ↑だとコンパイルが通り、↓だとエラーになる？
//       main.rsからはuse sound_compose_lib::internal::apps::cli::common::args;で参照できている。lib.rsからは？
// use common::AppError;

use crate::common::AppError::MyError;

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
/*
 * MEMO: 構造体
 *       C言語のイメージに近い。
 *       構造体のインスタンスが可変であれば、ドットでフィールドにアクセスできる。
 *       （一部のフィールドのみを可変にすることはできないことに注意）
 *       また、Javaのクラスのようにメソッドを一緒に定義することはできない。
 *       メソッドの定義は構造体を定義した後、implキーワードにより行う。
 *       https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/structs.html
 *       https://doc.rust-jp.rs/book/second-edition/ch05-01-defining-structs.html
 *       https://doc.rust-jp.rs/book/second-edition/ch05-02-example-structs.html
 *       // TODO: インスタンスの初期化の例（構造体名 {フィールド名: 初期値}）
 *       // TODO: impl、selfの有無によるインスタンスメソッド・staticメソッドの例と説明
 *                https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/method-syntax.html
 */
#[derive(Debug, StructOpt)]
// TODO: structoptの説明
//       https://stackoverflow.com/questions/50673567/how-to-use-an-enum-that-represents-subcommands-with-structopt
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
 *       https://qiita.com/dalance/items/56dba0dd54c82f937feb
 *       http://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/documentation.html
 */
/// 引数処理
pub fn args_proc() {
    // pretty_env_logger::init();

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
    // コマンドライン引数はStructOptで受け取るためログ出力のみ実施
    let args: Vec<String> = std::env::args().collect();
    debug!("{:?}", args);

    // StructOptで受け取った引数情報
    /*
     * MEMO: 変数宣言
     *       基本的に変数は「let 変数名」で宣言する（定数はconst）。
     *       デフォルトで不変なので、可変なのであれば「let mut 変数名」とする。
     *       型指定しない場合は推測してくれる。明示的に指定した場合は==の比較が厳密になる。
     *       https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/mutability.html
     *       https://23prime.hatenablog.com/entry/2018/11/27/143411
     *       https://qiita.com/aimof/items/c00b911ef5fb8cfed149
     */
    let mut opt = Opt::from_args();
    /* MEMO: 所有権
     *       以下のコメントアウトコードでopt2にoptの構造体の参照を代入すると、
     *       Javaでは2つの参照型変数から1つのオブジェクトを参照できたが、
     *       スコープの切れ目が曖昧になるため、Rustでは所有権が移動したとみなされ、
     *       古い参照型変数は使えなくなる。
     *       // TODO: println!で使えるのは何故？
     */
    // let opt2 = opt;
    println!("引数チェック前 {:?}", opt);

    /*
     * MEMO: if文
     *       条件式の"("と")"が不要である点以外は他の言語と変わらない。
     */
    if opt.verbose == true {
        // ログレベル
        env::set_var("RUST_LOG", "debug");
    }

    // 引数チェック処理
    // TODO: エラーだった場合の処理
    // TODO: ?で委譲できる？
    // TODO: ここで?が使えないのは何故？→ResultかOptionを返却する関数内でしか使えない
    check_opt(&mut opt);

    // TODO: ログレベルをデバッグにしているのにログが出力されない件
    info!("引数チェック後 {:?}", opt);
    println!("引数チェック後 {:?}", opt);

    // yamlファイルを読み込む
    let yaml_file = yaml::read_file(opt.file.clone().unwrap());

    let yaml_data = yaml::deserialize_file(yaml_file.unwrap());

    // TODO: 実装中なので後で消す
    println!("{:?}", yaml_data);

}

/*
 * MEMO: 借用
 *       関数の引数に構造体の参照を取ること。
 *       Javaでオブジェクトをメソッドの引数に渡す時のイメージ。
 *       以下関数は受け取った構造体の中身を変更する（ファイル名設定）ため、可変借用が必要。
 *       デフォルトは不変なので、型を「&mut 構造体名」としなければならない。
 *       不変借用で良い場合、型は&Optとなる。渡す変数はlet optで宣言し、
 *       参照は&optで渡すことで十分となる。
 *       なお、不変借用中は、返却されるまで不変な参照を含め他の参照は一切取得できない（並列性のため）。
 *       https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html
 *       https://doc.rust-jp.rs/book/second-edition/ch05-02-example-structs.html
 *       http://www.swlab.cs.okayama-u.ac.jp/~nom/lect/rust/The-Rust-Programming-Language-Summary-Chap3-8.html
 *       https://qiita.com/nebutalab/items/1d7a03c36c087c3f6360
 *       https://teratail.com/questions/114569
 */
/// 設定ファイル存在チェック
fn check_opt(opt: &mut Opt) -> std::result::Result<File, AppError::YamlFileNotFoundError> {
    // TODO: チェック処理なのでResultのTがいらない場合は()でいいのか
    //       https://doc.rust-lang.org/std/result/#results-must-be-used
    //       https://internals.rust-lang.org/t/implicit-void-result-ok/9863
    /*
     * MEMO: Option(列挙型)
     *       JavaのOptional型に近い。
     *       Rustはnull安全な言語(nullが原因で実行時エラーを起こさない性質)
     *       値がない可能性のある値はOption<T>で宣言する。
     *       値がある場合は値をラップした列挙値Some(value)、ない場合は列挙型Noneとなる。
     *       is_none()やis_some()で判定が可能。
     *       https://doc.rust-jp.rs/rust-by-example-ja/std/option.html
     *       https://nacika.com/entry/2017/11/12/071722/
     *       https://qiita.com/koher/items/e4835bd429b88809ab33
     */
    
    // TODO: エラーハンドリングについて
    //       https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199#thiserror
    //       https://3c1u.hatenablog.com/entry/2019/09/18/060000
    if opt.file.is_none() {
        // if File::open("sound-compose.yaml").is_ok() {
        //     opt.file = Some(String::from("sound-compose.yaml"));
        // } else if File::open("sound-compose.yml").is_ok() {
        //     opt.file = Some(String::from("sound-compose.yml"));
        // } else {
        //     // TODO: エラーにしなければならない
        //     // TODO: テスト追加

        //     opt.file = Some(String::from("sound-compose_none.yml"));
        // }
        // TODO: ifで判定するパターン
        let file_open_result = File::open("sound-compose.yaml");
        if file_open_result.is_ok() {
            opt.file = Some(String::from("sound-compose.yaml"));
            return Ok(file_open_result.unwrap());
        } else {
            // matchで判定するパターン
            match File::open("sound-compose.yml") {
                Ok(file_obj) => {
                    opt.file = Some(String::from("sound-compose.yml"));
                    return Ok(file_obj);
                },
                Err(err) => {
                    // TODO: 呼び出し元のエラー対応時に消すこと
                    opt.file = Some(String::from("sound-compose_none.yml"));

                    // TODO:returnは必要？
                    return Err(AppError::YamlFileNotFoundError(err));
                },
            }   
        }
    } else {
        // TODO: elseがないとエラーになる件
        // TODO: clone()しないとコンパイルエラーになる件、"cloneだらけ"で検索
        match File::open(opt.file.clone().unwrap()) {
            Ok(file_obj) => {
                return Ok(file_obj);
            },
            Err(err) => {
                return Err(AppError::YamlFileNotFoundError(err));
            },
        }   
    }

    // TODO: ソースが悪い気がする。
//    panic!("ここまででreturnされないことが想定外");
//    return Err(AppError::YamlFileNotFoundError(std::io::Error::new(ErrorKind::Other, MyError::new())));
}

// TODO: test用のuseはどうやって定義するの？
#[cfg(test)]
mod tests {
    /*
     * MEMO: テストモジュールのuse
     *       「use super::*;」しておけば、テスト対象モジュールで使用しているモジュールがそのまま使える。
     */
    use super::*;

    /*
     * MEMO: 定数
     *       
     */
    // lazy_static! {
    //     pub static const FILE_PATH: String = String::from("sound-compose-test.yaml");
    // }

    #[test]
    fn test_check_opt_is_some() {
        /*
         * MEMO: 自動テスト、テストのフェース
         *       https://sites.google.com/site/programmerscheatcheatcheat/software-test/yunittotesuto/junit/junitwo-ben-ge-deni-shimeru-qianni
         */

        // Setup
        /*
         * MEMO: 定数
         *       基本的にはletの代わりにconstを使用すればいい。Javaと同じく慣習的に大文字にする。
         *       型を明示しないとコンパイルエラーとなることに注意。
         *       https://doc.rust-jp.rs/rust-by-example-ja/custom_types/constants.html
         */
        // TODO: constで動かなかったので非定数としたが・・・定数にすればcloneもいらないはず
        let FILE_PATH = String::from("sound-compose-test.yaml");
        let file = std::fs::OpenOptions::new()
            .create(true).write(true).open(FILE_PATH.clone())
            .unwrap();

        let mut buildOpts = BuildOpts {
            no_cache: true,
        };

        let mut opt = crate::internal::apps::cli::common::args::Opt {
            file: Some(FILE_PATH.clone()),
            sub: Sub::Build(buildOpts),
            verbose: true,
        };

        let expected = FILE_PATH.clone();

        // Run
        #[warn(unused_must_use)]
        check_opt(&mut opt);

        // Verify
        assert_eq!(expected, opt.file.unwrap());

        // Teardown
        std::fs::remove_file(FILE_PATH);
    }
}