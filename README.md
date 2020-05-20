# sound-compose
super simplified DAW app like a docker-compose

# TODO for dev
* VisualStudio 2019 Communityのインストール
* QT 5.14.0のインストール（rust-qtが対応しているのがこのバージョンまで）
* （要確認）CMakeのインストール
+ SDL2関連のDDL配置（make/dll/sdl2/*.dll）

# 気になる
* 条件付きコンパイル（Rust公式ドキュメント）
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/conditional-compilation.html

* [Rust] web-viewでGUIアプリをつくる
https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd

* これがなくては生きていけないVS Codeエクステンション10選
https://qiita.com/rana_kualu/items/9f6919311f1407a71c5f

* 共有メモリ (メモリ マップト ファイル) を利用する - C#プログラミング
https://www.ipentec.com/document/csharp-use-memory-mapped-file

* rustでtailモドキを作ってみた
https://qiita.com/instance0000/items/d4ad45101e9208887ee0

* 週アレ（５）　APIから見る共有メモリ（前編）
http://fe0km.blog.fc2.com/blog-entry-83.html

* Rustのパターンっぽいやつメモ
https://gist.github.com/qnighy/be99c2ece6f3f4b1248608a04e104b38



# vscodeでやること
* 拡張機能「Rust(rls内包)」、<s>「Native Debug」</s>「CodeLLDB」
※以下サイトがわかりやすい
https://qiita.com/ousttrue/items/ee617544ab737fc34c1d
※ただしデバッグはCodeLLDBを使う。Run＞Add Configuration...でLLDBが2つ出てきた。
　Cargo.tomlを検知して内容を生成してくれる方を使う（もう片方はlldb-mi）
※初回のCodeLLDBは「Debug executable 'プロジェクト名'」実行後に結構待たされる。
　vscodeの下部のメニューバーに以下が出ている間は待っていれば良いはず。
「▷Debug executable 'プロジェクト名' (プロジェクト名)  Rust(回る歯車)[プロジェクト名] Starting」と

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

* RustのLinux muslターゲット （その1：Linux向けのポータブルなバイナリを作る）
https://blog.rust-jp.rs/tatsuya6502/posts/2019-12-statically-linked-binary/

# なう
* Command Line Toolを作ってみる in Rust
https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c
※テスト、エラーハンドリング（unwrap？failure？thiserror？anyhow？）、パイプ処理（いったん使えないようにして考慮しなくてよくする？）、CI、ランチャー、「次にどんなことをすれば」が未。CIは後日？Travis CIの利点は？

* エラー処理（公式ドキュメント）
https://doc.rust-jp.rs/book/second-edition/ch09-00-error-handling.html

* Rustのエラーとなかよくなる
https://3c1u.hatenablog.com/entry/2019/09/18/060000
※じぶんでエラー型を定義する→済
※Fromトレイトを実装する→未

* rustのエラー処理と疑問符演算子
https://cat-in-136.github.io/2018/04/rust-error-handling-question.html

* Rustのエラーハンドリング
https://medium.com/@11Takanori/rust%E3%81%AE%E3%82%A8%E3%83%A9%E3%83%BC%E3%83%8F%E3%83%B3%E3%83%89%E3%83%AA%E3%83%B3%E3%82%B0-6660cd4d16c0

* Rustの便利クレート
https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199
※thiserror、anyhowについて
※maplit、getset、im、typenum、remove_dir_all、which、pretty_assertions、difference、tempdir

* Result について
http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
※unwrap()

* エラーハンドリング（公式ドキュメント）
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/error-handling.html#result-%E5%9E%8B

* タプル構造体
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/structs.html#%E3%82%BF%E3%83%97%E3%83%AB%E6%A7%8B%E9%80%A0%E4%BD%93

* Rustのイディオム
https://qiita.com/MoriokaReimen/items/b32ebcb0ab23bffd7e9b

* Rustのstruct、traitの使い方
https://qiita.com/yasuyuky/items/8894f731da9a4e8cac4c

* RustでOption値やResult値を上手に扱う
https://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a

* Rustのエラー処理
https://qiita.com/fujitayy/items/cafe661415b6aa33d884
※?オペレーター

* 【Rust】がばがばRust独学 - 9. Error Handling
https://charaken.hatenablog.com/entry/2020/01/06/070000

* RustのOptionとResult
https://qiita.com/take4s5i/items/c890fa66db3f71f41ce7

* The Rust Programming Language 要約 3-8章
http://www.swlab.cs.okayama-u.ac.jp/~nom/lect/rust/The-Rust-Programming-Language-Summary-Chap3-8.html

*Rust1.0学習用私的メモ
https://qiita.com/yohhoy/items/e78dcc4d168f247d83ce

* リファクタリングしてモジュール性とエラー処理を向上させる
https://doc.rust-jp.rs/book/second-edition/ch12-03-improving-error-handling-and-modularity.html

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

* rustで作るcli tool
https://qiita.com/syui/items/e071ba72ea82d583e380
※test、makefile、CIが未。CIは後日？Travis CIの利点は？

# 済（Makefile）
* rustで作るcli tool
https://qiita.com/syui/items/e071ba72ea82d583e380
※面倒なコマンドは、makefile化することもできます。

* Windowsでmakeコマンドを使う
https://qiita.com/tokikaze0604/items/e13c04192762f8d4ec85

* WindowsでもUNIXでも動くMakefileのポイント
https://qiita.com/bottomzlife/items/bf36b4d057bd22e54a46

* シンプルで応用の効くmakefileとその解説
http://urin.github.io/posts/2013/simple-makefile-for-clang

・Makefileでrunするときに引数を渡す
https://qiita.com/tortuepin/items/9861c75853b516c8a279

# 済（Github Action）
* GitHub ActionsでReleaseを自動化する方法としたときに得た学び
http://skawashima.com/blog/2019/12/github-actions-auto-release/

* Rust cargo Action
https://github.com/actions-rs/cargo

* GitHub ActionsでRustプロジェクトをクロスビルドしてリリースする
https://motemen.hatenablog.com/entry/2019/11/github-actions-crossbuild-rust

* Rustでクロスコンパイル
https://ryochack.hatenablog.com/entry/2017/10/22/014735

* RustのLinux/Windows/macOS向け64bitバイナリをGitHub Actionsで生成する
https://qiita.com/dalance/items/66d97c252b8dd9c96c29

# 済（SDL2）
* toyamaguchi / rust_opengl（GitHub）
https://github.com/toyamaguchi/rust_opengl

* SDLインストール
https://github.com/Rust-SDL2/rust-sdl2
http://www.libsdl.org/download-2.0.php
[1]:SDL2-devel-2.0.12-VC.zipをダウンロードし解答
[2]:
SDL2-devel-2.0.12-VC.zip\SDL2-2.0.12\lib\x64
の中身を
%USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
にコピー
→「コンピュータに SDL2.dll がないため、プログラムを開始できません。この問題を解決するには、プログラムを再インストールしてみてください。」エラーが発生する。ビルドで生成されたexeファイルと同階層にSDL2のdllファイル群があればえらーが発生しない。手順だと、rustupのtoolchainsフォルダの奥底に格納すればいいはずだが・・・
[3]~/.cargo/config
[target.x86_64-pc-windows-msvc] 
rustflags = ["-C", "link-args=-static"] 
→うまくいかず・・・ひとまず以下手順でdllを同じファイルにコピーすることとする。
★SDL2暫定開発手順（SDL2アプリを動かす場合は必須）
①プロジェクト直下にmakeフォルダ、その中にdll/SDL2フォルダを作成し、
　「SDL2-devel-2.0.12-VC.zip\SDL2-2.0.12\lib\x64」の中身をコピー
（makeフォルダは.gitignoe追加済）
②make debug OPT="～～～"　を実行

* WindowsでRacerのビルド（コンパイル）
https://silight.hatenablog.jp/entry/2015/07/07/151146

* Rust でしっかりとスタティックリンク
https://qiita.com/moriai/items/b1fa7d1b43d985d408cc

* 静的にリンクされた実行ファイルを生成するには？
https://stackoverrun.com/ja/q/8748801

★「Static linking with MSVC」の意味の理解が必要そう。

# なう（Qt）
* Qt（オープンソース版）をオンラインインストーラーでインストール
http://qt-users.jp/download.qml
※自宅のWindows 7(64bit)では「qt-unified-windows-x86-3.2.2-online.exe」を実行した。

* Setting up（Rust + Qt guide）
https://rust-qt.github.io/qt/setting_up/
※C++ compilerはRustの環境構築時にインストールした「Visual C++ビルドツール」で良いかもしれないので一旦入れないことにした・・・
※パスに「C:\Qt\5.14.2\msvc2017_64\bin」を追加

* CMake
https://cmake.org/download/
cmake-3.17.2-win64-x64.msi
※インストーラー内の手順でPath追加を行うこと。

* エラー発生中
C:\Users\sou>mkdir cd C:\tmp

C:\Users\sou>cd C:\tmp

C:\tmp>mkdir build

C:\tmp>cd build

C:\tmp\build>qmake C:\Qt\Examples\Qt-5.14.
Project ERROR: Cannot run compiler 'cl'. O
===================
===================
Maybe you forgot to setup the environment? 





https://github.com/rust-qt/examples/tree/master/widgets/basic_form

CMAKE、VC++コンパイラ、qmakeのパスを通す

C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.23.28105\bin\Hostx64\x64
C:\Qt\Qt5.14.1\5.14.1\mingw73_64\bin
https://tomokiit.hatenablog.jp/entry/2018/04/16/195947

QtがMinGWしかなかったので、MSVCも入れてみる。
※以下が発生した
・【Qt】Qt Maintenance Toolで「このアクションの実行にはひとつ以上の有効なリポジトリが必要です。」と表示される。
https://tomokiit.hatenablog.jp/entry/2018/04/16/195947

※Qt Maintenance ToolではMSVCの追加ができなかったため、Qtのオンラインインストーラーから追加した。
qt-unified-windows-x86-3.2.2-online.exe
MSVC 2017 64-bitのみチェックしインストール
→後で、Qtから始まるものと、デフォルト選択のQt CreatorとDebbuging Tools for Windowsも選択して入れなおした。

C:\_dev\cpp\B-Synth
にclone
http://gnuwin32.sourceforge.net/packages/make.htm
インストール後以下パス追加
C:\Program Files (x86)\GnuWin32\bin

http://qt-log.open-memo.net/sub/first__how_to_use_QT.html

cd C:\_dev\cpp\B-Synth
qmake -project
qmake
make release

C:\_dev\cpp\B-Synth>make release
make -f Makefile.Release
make[1]: ディレクトリ `C:/_dev/cpp/B-Synth' に入ります
Makefile.Release:76: *** 分離記号を欠いています.  中止.
make[1]: ディレクトリ `C:/_dev/cpp/B-Synth' から出ます
make: *** [release] エラー 2

タブ文字漏れのようで、以下の通り対応していく

{release}.cpp{release\}.obj::
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -Forelease\ @<<
	$<
<<
↓
{release}.cpp{release\}.obj::
	$(CXX) -c $(CXXFLAGS) $(INCPATH) -Forelease\ @<<
	$<
	<<

C:\_dev\cpp\B-Synth>make release
make -f Makefile.Release
make[1]: ディレクトリ `C:/_dev/cpp/B-Synth' に入ります
cl -c -nologo -Zc:wchar_t -FS -Zc:rvalueCast -Zc:inline -Zc:strictStrings -Zc:throwingNew -Zc:referenceBinding -Zc:__cplusplus -O2 -MD -W3 -w34100 -w34189 -w44996 -w44456 -w44457 -w44458 -wd4577 -wd4467 -EHsc -DUNICODE -D_UNICODE -DWIN32 -D_ENABLE_EXTENDED_ALIGNED_STORAGE -DWIN64 -DNDEBUG -DQT_NO_DEBUG -DQT_GUI_LIB -DQT_CORE_LIB -I. -I. -I..\..\..\Qt\5.14.2\msvc2017_64\include -I..\..\..\Qt\5.14.2\msvc2017_64\include\QtGui -I..\..\..\Qt\5.14.2\msvc2017_64\include\QtANGLE -I..\..\..\Qt\5.14.2\msvc2017_64\include\QtCore -Irelease -I/include -I..\..\..\Qt\5.14.2\msvc2017_64\mkspecs\win32-msvc  -Forelease\ @<<
<< の使い方が誤っています。
make[1]: *** [{release}.cpp{release\}.obj] エラー 255
make[1]: ディレクトリ `C:/_dev/cpp/B-Synth' から出ます
make: *** [release] エラー 2

※makeの変わりにqmake（タブ文字漏れの対応は戻さないといけない）

qt-unified-windows-x86-3.2.2-online.exe
MSVC 2017 64-bitのみチェックしインストール
→後で、Qtから始まるものと、デフォルト選択のQt CreatorとDebbuging Tools for Windowsも選択して入れなおした

Qt CreatorにてB-Synthのエラーが発生した。
うまく動かなかったのでプロジェクトをMSVCで作り直した・・・「sound-compose-gui」
適当なパスに作成したプロジェクトをコピーし中に移動
mkdir bin
cmake .
cmake --build .
「Debug\sound-compose-gui.exe」をダブルクリックし、ウィンドウが表示されればOK

https://stackoverflow.com/questions/42881758/cmake-does-not-produce-exe-file

結局rust-qtの「qt_basic_form」のサンプルは動かず。




■再整理
・VC++コンパイラはRust環境設定時に済
・Qtインストール（MSVC 2017）、MSVCで作成した空プロジェクトが起動できることを確認
・CMakeインストール、パスを通す
・システム環境変数の追加
C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.23.28105\bin\Hostx64\x64
C:\Qt\5.14.2\msvc2017_64\bin
C:\Qt\Qt5.14.1\5.14.1\mingw73_64\bin
・「https://github.com/rust-qt/examples」を実行
cd C:\_dev\rust\cargo
git clone https://github.com/rust-qt/examples.git
cd examples
cargo run --bin basic_form

・Visual Studio 2017 統合環境で発生する “rc.exe が見つかりません” への対応
https://www.xlsoft.com/jp/blog/intel/2017/07/03/visual-studio-2017-%E7%B5%B1%E5%90%88%E7%92%B0%E5%A2%83%E3%81%A7-rc-exe-%E3%81%8C%E8%A6%8B%E3%81%A4%E3%81%8B%E3%82%8A%E3%81%BE%E3%81%9B%E3%82%93-%E3%81%AE%E5%AF%BE%E5%BF%9C/
↓以下をシステム環境変数に設定
C:\Program Files (x86)\Windows Kits\10\bin\10.0.18362.0\x64
→ダメ

・MinGWのld.exeを使っている件
システム環境変数から以下を削除
C:\MinGW\bin\
→ダメ


★GTKお試し
* WindowsでRust環境を作ってGtk3でOpenGLする
https://qiita.com/ousttrue/items/ee617544ab737fc34c1d
※採用

* MSYS2 installer
https://www.msys2.org/
[1]：msys2-x86_64-20190524.exeをダウンロードし実行、インストール
[2]：C:\msys64\usr\bin\mintty.exeを実行→MSYS2
[3]：pacman --needed -Sy bash pacman pacman-mirrors msys2-runtime
[4]：pacman -S mingw-w64-x86_64-gcc
[5]：pacman -S mingw-w64-x86_64-gdb
[6]：pacman -S mingw-w64-x86_64-gtk3
→インストールに失敗する・・・頓挫
※python3は削除しないほうがいい・・・「CodeLLDB」の動作で必要になるため。
[7]：「C:\msys64\mingw64\bin」をシステム環境変数Pathに追加（vscodeやコマンドプロンプトは再起動）

* MSYS2 による gcc 開発環境の構築
https://qiita.com/spiegel-im-spiegel/items/ba4e8d2418bdfe0c8049
※MSYS2の使い方の参考にした。この通りには実施していない。

* Windows で GTK+, Glade のインストール（MSYS2 を使用）
https://www.kkaneko.jp/tools/win/gtkplus.html
※MSYS2の使い方の参考にした。この通りには実施していない。







■Qtのコマンドラインビルド
git clone https://github.com/sohrin/sound-compose-gui.git
cd sound-compose-gui
cmake .
cmake --build .

■cloneしたQtプロジェクトのQt Creatorへのインポート
ビルド時にmsvcやmingwを間違えなければ大丈夫そう




■Conrod
* PistonDevelopers / conrodGitHub（GitHub）
https://github.com/PistonDevelopers/conrod

* サンプルソース
https://github.com/PistonDevelopers/conrod/tree/master/backends/conrod_glium/examples

* サンプル実行コマンド
https://docs.rs/conrod/0.61.1/conrod/guide/chapter_2/index.html#running-the-conrod-examples
※conrodフォルダ直下で実施
cargo run --release --features "winit glium" --example all_winit_glium
cargo run --release --features "winit glium" --example canvas
cargo run --release --features "winit glium" --example primitives
cargo run --release --features "winit glium" --example text

* gliumライブラリについて
http://onagat.hatenablog.com/entry/2017/04/28/012532
https://github.com/rust-windowing/glutin
https://docs.rs/glutin/0.17.0/glutin/index.html





■MSVC関連
https://rust-qt.github.io/qt/setting_up/
↓
* (2019年版)コマンドラインからclを使ってコンパイルできるようにしたので共有する
https://qiita.com/asana_yui/items/d545b5eccd994c0cdaab
※Visual Studioを上記サイトの通りインストール、MSVC、Windows 10 SDKが重要そう
↓
x64 Native Tools Command Prompt for VS 2017
で、rust-qtのSetting upのVerifying installationが動いた。
↓
https://rust-qt.github.io/qt/getting_started/
↓
https://github.com/rust-qt/examples/blob/master/widgets/basic_form/Cargo.toml
https://github.com/rust-qt/examples/blob/master/widgets/basic_form/src/main.rs
↓
Caused by:
  process didn't exit successfully: `C:\_dev\rust\sound-compose\target\debug\build\qt_widgets-94e6d9
df575f5371\build-script-build` (exit code: 1)
--- stdout
Current Qt version (5.14.2) is unknown to qt_widgets crate. Using closest known version (5.14.0)
Current target is unknown: v5.14.0 on x86_64-windows-windows-msvc
↓
Qtをv5.14.0で再インストールする必要がありそう（念のためMSVC 2017とUWP 2017の両方を入れた）
※Qtへの環境変数Pathも変わるため注意
C:\Qt\5.14.2\msvc2017_64\bin
↓
C:\Qt\5.14.0\msvc2017_64\bin





■FFI
* Rustによるダイナミックライブラリの作り方と他言語からの呼び出し
https://qiita.com/yasuyuky/items/45f7333118fa165b670b

* ダイナミック リンク ライブラリ（DLL）の基礎知識
http://exlight.net/devel/windows/dll/windll.html

* Rust の crate_type をまとめてみた
https://qiita.com/etoilevi/items/4bd4c5b726e41f5a6689

* FFI を使って Ruby から Rust の関数を呼び出す
https://blog.mzumi.com/post/2016/10/18/hello-ffi/
https://blog.mzumi.com/post/2016/11/04/ffi-samples/
https://blog.mzumi.com/post/2016/11/14/ffi-pointer-samples/

* QtのDynamic Link
http://qt-log.open-memo.net/sub/system__use_dynamic_library(DLL).html

* Rustで実装したアルゴリズムをUnityから使う
https://qiita.com/hadashiA/items/3755786e95bbcd8f3b5d

* Rustオブジェクトの削除タイミングを手動で制御する
https://qiita.com/tatsuya6502/items/b9801d92f71e24874c9d
※FFI（他言語関数インターフェイス）経由で、Rust のオブジェクトを他の言語に渡したい時

* 【QLibrary】DLL内の関数を呼び出す
https://dnaga392.hatenablog.com/entry/2015/06/11/000114

* ライブラリのリンク方法をきっちり区別しよう
https://qiita.com/argama147/items/2f636a2f4fd76f6ce130