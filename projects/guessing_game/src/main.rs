// 外部依存ファイル、rand::で呼び出せるようになる
extern crate rand;

// スコープに導入する
use std::io; // std:標準ライブラリ、io:入出力ライブラリ
use std::cmp::Ordering;
use rand::Rng; //トレイト（詳細は第９章）

/*
エントリーポイント
fn:関数、()引数なし、{}で開始と終わり（スコープ）
*/
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    /*
    thread_rng() → 乱数発生器
    gen_range() → Rngトレイトで定義されたメソッド、２つの数字間で乱数発生させる
    */

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        /*
        別例
        let foo = bar;
        fooにbarをbind。Rustの変数は標準で不変（immutable）。

        let foo = 5;
        let mut bar = 5;
        mutで変数を可変に。
        */

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        /*
        io::stdin() → use std::io;していなければstd::io::stdin
                    ターミナルへの標準入力へのハンドルを表す
        read_line(&mut guess) → ユーザから入力を受け付ける。&mut guessは引数
        & → reference（参照）。データを複数回メモリにコピーせずとも、
            コードの複数箇所で同じデータにアクセス可能になる。
            referenceも標準でimmutableのためmutableにするにはmutを付ける。
        io::Result → read_lineの結果、io::Resultの型（列挙型、enumと呼ばれる）で返却値(中身はOkかErr)が返される。
                    メソッドが定義されており、expectメソッドがある。
        expect → Errの場合、プログラムをクラッシュ、Okの場合、引数として渡されたメッセージを表示。
                io::Resutの戻り値を使わずにコンパイル使用とすると、戻り値未使用で警告がでる。
                    
        */

        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        /*
        guessをshadow（シャドーウィング） → guessの値を覆い隠す。
        　　　　　　　　　　　　　　　　　　　 別々の変数を２つ作らされることなく、guessを再利用（詳細は第3章）
        trim → 両端空白を取り除く。ここでは入力後の\nを取り除く。
        parse → 文字列を解析して何らかの数値にする。Result型(Ok、Err)が返却。
        : → コンパイラに変数の型を注釈する合図
        u32 → 32ビットの非負整数
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        /*
        {} → プレースホルダー。
        let x = 5;
        let y = 10;
        println!("x = {} and y = {}", x, y);
        */

        println!("Please input your guess.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Toobig!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        /*
        Ordering → enum（Less,Greater,Equal）
        match → 複数のアーム（腕）でできている。matchに与えられた値を取り、
                各アームのパターンを順番に照合する。（詳細は第6章、第18章）
        cmpの結果、例えばOrdering::Greaterであれば、matchはOrdering::Lessから順番に照合していく
        */
    }
}
