## 自習

- 日本語ドキュメント
  - https://doc.rust-jp.rs/
  - https://doc.rust-jp.rs/book/second-edition/ch01-01-installation.html
- 英語ドキュメント
  - https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
- github
  - https://github.com/rust-lang/rustlings
- VS Code
  - Rust(rls)
    - https://github.com/rust-lang/rls-vscode
    - https://tech-blog.optim.co.jp/entry/2019/07/18/173000
  - Better TOML
    - https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
- 参考
  - https://imoz.jp/note/rust-functions.html

1. 事始め
    1. インストール
      ```

      xxxx$ curl https://sh.rustup.rs -sSf | sh
      info: downloading installer

      Welcome to Rust!

      This will download and install the official compiler for the Rust
      programming language, and its package manager, Cargo.

      It will add the cargo, rustc, rustup and other commands to
      Cargo's bin directory, located at:

        /Users/xxxx/.cargo/bin

      This can be modified with the CARGO_HOME environment variable.

      Rustup metadata and toolchains will be installed into the Rustup
      home directory, located at:

        /Users/xxxx/.rustup

      This can be modified with the RUSTUP_HOME environment variable.

      This path will then be added to your PATH environment variable by
      modifying the profile files located at:

        /Users/xxxx/.profile
      /Users/xxxx/.bash_profile

      You can uninstall at any time with rustup self uninstall and
      these changes will be reverted.

      Current installation options:


        default host triple: x86_64-apple-darwin
          default toolchain: stable
                    profile: default
        modify PATH variable: yes

      1) Proceed with installation (default)
      2) Customize installation
      3) Cancel installation
      >1
      info: profile set to 'default'
      info: syncing channel updates for 'stable-x86_64-apple-darwin'
      info: latest update on 2019-11-07, rust version 1.39.0 (4560ea788 2019-11-04)
      info: downloading component 'cargo'
        3.7 MiB /   3.7 MiB (100 %)   1.4 MiB/s in  2s ETA:  0s
      info: downloading component 'clippy'
        1.3 MiB /   1.3 MiB (100 %) 831.1 KiB/s in  1s ETA:  0s
      info: downloading component 'rust-docs'
      11.8 MiB /  11.8 MiB (100 %)   1.0 MiB/s in 10s ETA:  0s
      info: downloading component 'rust-std'
      173.7 MiB / 173.7 MiB (100 %)   3.8 MiB/s in 53s ETA:  0s
      info: downloading component 'rustc'
      60.2 MiB /  60.2 MiB (100 %)   3.4 MiB/s in 18s ETA:  0s
      info: downloading component 'rustfmt'
      info: installing component 'cargo'
      info: installing component 'clippy'
      info: installing component 'rust-docs'
      11.8 MiB /  11.8 MiB (100 %)   2.8 MiB/s in  3s ETA:  0s
      info: installing component 'rust-std'
      173.7 MiB / 173.7 MiB (100 %)  25.9 MiB/s in  7s ETA:  0s
      info: installing component 'rustc'
      60.2 MiB /  60.2 MiB (100 %)  13.1 MiB/s in  4s ETA:  0s
      info: installing component 'rustfmt'
      info: default toolchain set to 'stable'

        stable installed - rustc 1.39.0 (4560ea788 2019-11-04)


      Rust is installed now. Great!

      To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
      environment variable. Next time you log in this will be done
      automatically.

      To configure your current shell run source $HOME/.cargo/env

      xxxx:rust_study xxxx$ rustc --version
      rustc 1.39.0 (4560ea788 2019-11-04)
      ```
    2. Hello World
        - vscode
          - Rust(rls)
            - https://github.com/rust-lang/rls-vscode
            - https://tech-blog.optim.co.jp/entry/2019/07/18/173000
        - 自動整形
          - rustfmt
            - `rustfmt main.rs`
            - https://github.com/rust-lang/rustfmt
            - https://qiita.com/wordijp/items/6860e27643ba9d06a987
        - println!はRustのマクロ、関数はprintln
        - 行末は;
        - コンパイルコマンド`rustc main.rs`
    3. Hello, Cargo!
        - cargo
          - Rustのビルドシステム兼、パッケージマネージャ
            - https://doc.rust-lang.org/cargo/
          ```
          xxxx:hello_world xxxx$ cargo --version
          cargo 1.39.0 (1c6ec66d5 2019-09-30)
          ```
        - Cargoでのプロジェクト作成
          - `cargo new project_name --bin`
            - `--bin`は、Cargoにバイナリ生成プロジェクトを作成させる
          ```
          xxxx:projects xxxx$ cargo new hello_cargo --bin
          Created binary (application) `hello_cargo` package
          ```
          - Cargo.toml
            - Cargoの設定フォーマット
          - ソースファイルはsrcディレクトリに
          - ディレクトリ最上位
            - Readmeファイル、ライセンス情報、設定ファイル、他のコードに関連しないもの
        - Cargoプロジェクトをビルドし、実行
          - `cargo build`：target/debug/.に実行可能ファイルを作成
          ```
          xxxx:hello_cargo xxxx$ cargo build
          Compiling hello_cargo v0.1.0 (/Users/xxxx/Desktop/rust_study/projects/hello_cargo)
          Finished dev [unoptimized + debuginfo] target(s) in 3.58s
          xxxx:hello_cargo xxxx$ ./target/debug/hello_cargo
          Hello, world!
          ```
          - Cargo.lockファイル：自分のプロジェクトの依存の正確なバージョン管理
          - `cargo run`：コードをコンパイルかつ実行
            - コードに変更があった場合は再ビルドする
          ```
          xxxx:hello_cargo xxxx$ cargo run
              Finished dev [unoptimized + debuginfo] target(s) in 0.02s
              Running `target/debug/hello_cargo`
          Hello, world!
          ```
          - `cargo check`：コード確認、コンパイル可能かチェック、ただし実行可能ファイルは生成しない、その代わり`cargo build`より遥かに早い
        - リリースビルド
          - `cargo build --release`：target/relase/.に実行可能ファイルを作成
2. 数当てゲームをプログラムする
  - ソースコードのコメント中以外に日本語文字があるとコンパイルに失敗することがあるそう。文字列の英語は、コメントに和訳を載せる。重複する場合は最初の１回だけ掲載。
  - `std::prelude`：デフォルトで使えるもの（いくつかの型が入っている）
    - https://doc.rust-lang.org/stable/std/prelude/index.html
  - 1methodで1行ずつ（各methodで改行する）
  - crate（クレート）
    - Rustコードパッケージ
    - ライブラリクレート（現在作っているのはバイナリクレート）
    - https://crates.io/
    - tomlファイルに追記後、`cargo build`
    ```
    xxxx:guessing_game xxxx$ cargo build
        Updating crates.io index
      Downloaded rand v0.7.2
      Downloaded getrandom v0.1.13
      Downloaded rand_core v0.5.1
      Downloaded rand_chacha v0.2.1
      Downloaded libc v0.2.66
      Downloaded cfg-if v0.1.10
      Downloaded c2-chacha v0.2.3
      Downloaded ppv-lite86 v0.2.6
      Compiling libc v0.2.66
      Compiling getrandom v0.1.13
      Compiling cfg-if v0.1.10
      Compiling ppv-lite86 v0.2.6
      Compiling c2-chacha v0.2.3
      Compiling rand_core v0.5.1
      Compiling rand_chacha v0.2.1
      Compiling rand v0.7.2
      Compiling guessing_game v0.1.0 (/Users/xxxx/Desktop/rust_study/projects/guessing_game)
        Finished dev [unoptimized + debuginfo] target(s) in 32.85s
    ```
    - クレートダウンロード→コンパイル→プロジェクトコンパイル
      - 何も変更せずに即座に`cargo build`してもFinishedのみ表示
      - cargoは変更を検知してコンパイルを実行するか判断している
      - src/main.rsファイルに些細な変更をして再度ビルドしてもmain.rsのみ再ビルドされる
    - Cargo.lockファイル：最初にbuildした時のバージョンを保てるように
    - `cargo update`
      - tomlファイルのバージョン通りにクレートupdateする
      - バージョンは`0.3.15`と記載されていれば、0.3.0より大きく、0.4.0未満のバージョンのみを検索します(gemと同じ要領）
    - `cargo doc --open`：クレートの使用方法をブラウザで確認
    - Rustの標準の数値型は`i32`（`i32`:32ビットの数字、`u32`:32ビットの非負数字、`i64`:64ビットの数字）
    - 数値と文字列は比較できない

## rust-study channel memo
> 概要に関して最近記事書かれていました
https://qiita.com/evid/items/adfa237f4dc6b73a2b5d

> https://github.com/rust-lang/rustlings
https://doc.rust-lang.org/book/
https://qiita.com/advent-calendar/2019/rust
https://www.amazon.co.jp/%E5%AE%9F%E8%B7%B5Rust%E5%85%A5%E9%96%80-%E8%A8%80%E8%AA%9E%E4%BB%95%E6%A7%98%E3%81%8B%E3%82%89%E9%96%8B%E7%99%BA%E6%89%8B%E6%B3%95%E3%81%BE%E3%81%A7-%CE%BAeen/dp/4297105594 （編集済み） 

> チュートリアルは和訳もされているみたいですね(章番号は原語版と同じじゃないみたいですが)
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/

> 勉強会：LT祭り形式、2-3人/回 （編集済み） 
言語仕様
Rustを使うメリット
エコシステム
