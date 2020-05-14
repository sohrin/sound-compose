# sound-compose
super simplified DAW app like a docker-compose


# 未
* Rustプロジェクトのディレクトリ構成
https://qiita.com/dalance/items/a49dfbfdb0e5de39d908
※複数バイナリを生成する場合に確認する。

* Rust のモジュールシステム
https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
※複数プロジェクトでworkspaceを使うかgitの依存関係を使うか検討する。

* Rust Cargo でプロジェクト管理、リリースビルドの方法
https://webbibouroku.com/Blog/Article/rust-cargo#outline__4_1

* RustでデータをJSONやYAML等にserialize/deserializeする方法
https://qiita.com/fujitayy/items/ed0033f8bb036d23710c

* serdeの機能で様々な形態のJSONを列挙型として扱う
https://igaguri.hatenablog.com/entry/2018/12/28/120500

* RustのSerdeの簡単な紹介
https://qiita.com/garkimasera/items/0442ee896403c6b78fb2

* Rustのstruct、traitの使い方
https://qiita.com/yasuyuky/items/8894f731da9a4e8cac4c

* Rustのモジュールシステムが厳格
https://kk-river108.hatenablog.com/entry/2018/12/02/222329

* Rust のエラーハンドリングはシンタックスシュガーが豊富で完全に初見殺しなので自信を持って使えるように整理してみたら完全に理解した
https://qiita.com/nirasan/items/321e7cc42e0e0f238254

* Documentation tests
https://doc.rust-lang.org/rustdoc/documentation-tests.html

* Rustのパニック機構
https://qnighy.hatenablog.com/entry/2018/02/18/223000

* Rustのderiveはあまり頭がよくない
https://qnighy.hatenablog.com/entry/2017/06/01/070000

* if letで簡潔なフロー制御
https://doc.rust-jp.rs/book/second-edition/ch06-03-if-let.html

* Rustのパターンマッチを完全に理解した
https://frozenlib.net/blog/2018-03-11_rust-pattern-match/

* アンダースコア(Rustの基本構文-3-)
https://qiita.com/jin237/items/59ef229a4de30cb8203b

* What does the error `cannot be named the same as a tuple variant` mean?
https://stackoverflow.com/questions/46065487/what-does-the-error-cannot-be-named-the-same-as-a-tuple-variant-mean


# なう
* 【Rust】がばがばRust独学 - 9. Error Handling
https://charaken.hatenablog.com/entry/2020/01/06/070000

* RustのOptionとResult
https://qiita.com/take4s5i/items/c890fa66db3f71f41ce7

* The Rust Programming Language 要約 3-8章
http://www.swlab.cs.okayama-u.ac.jp/~nom/lect/rust/The-Rust-Programming-Language-Summary-Chap3-8.html

* rustで作るcli tool
https://qiita.com/syui/items/e071ba72ea82d583e380
※test、makefile、CIが未。CIは後日？Travis CIの利点は？

* Command Line Toolを作ってみる in Rust
https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c
※テスト、エラーハンドリング（unwrap？failure？thiserror？anyhow？）、パイプ処理（いったん使えないようにして考慮しなくてよくする？）、CI、ランチャー、「次にどんなことをすれば」が未。CIは後日？Travis CIの利点は？

*Rust1.0学習用私的メモ
https://qiita.com/yohhoy/items/e78dcc4d168f247d83ce

* リファクタリングしてモジュール性とエラー処理を向上させる
https://doc.rust-jp.rs/book/second-edition/ch12-03-improving-error-handling-and-modularity.html

* Rustの便利クレート
https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199
※maplit、getset、im、typenum、remove_dir_all、which、pretty_assertions、difference、tempdir、

* Rustライブラリとバイナリの両方を含むパッケージ？
https://www.it-swarm.dev/ja/rust/rust%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E3%81%A8%E3%83%90%E3%82%A4%E3%83%8A%E3%83%AA%E3%81%AE%E4%B8%A1%E6%96%B9%E3%82%92%E5%90%AB%E3%82%80%E3%83%91%E3%83%83%E3%82%B1%E3%83%BC%E3%82%B8%EF%BC%9F/1050664534/

* Rustのモジュールの使い方
https://keens.github.io/blog/2017/01/15/rustnomoju_runokirikata/
※私がRustを書く時はmain.rsの中にmodを書くことはないです。必ずlib.rsを作って、そこでライブラリとしてまとめてからmain.rsで使います。

* 実践的なアプリケーションを書いてみよう！ Rustの構造化プログラミング【第二言語としてのRust】
https://employment.en-japan.com/engineerhub/entry/2017/07/19/110000
※Option型、Result型、トレイトがまだ分かっていない。





# 済
* Goにはディレクトリ構成のスタンダードがあるらしい。
https://qiita.com/sueken/items/87093e5941bfbc09bea8