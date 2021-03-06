# rust-exercise

## ノート
* `String::new()` の `::` は new が String 型の関連関数であることを表している。スタティックメソッドと呼ぶ言語もある
* `read_line()` の引数には、格納する文字列を渡す
* `parse()` は何にパースするか明示する必要がある場合が多い
    * guess: u32 としても、parse::<u32>() としてもよい
    ```rust
    let guess: u32 = guess.trim().parse::<u32>().expect("")
    ```
* トレイトに実装されてるメソッドがある場合はトレイトのインポートが必要
    ```rust
    use rand::Rng; // これ
    let secret_number = rand::thread_rng().gen_range(1, 11);
    ```
* 定数 `const`: 可変にはできない。型注釈は必ず必要。グローバルスコープである。定数式しかセットできない（関数呼び出しの結果や実行時に評価される値はだめ）
    * グローバルスコープとのことで、あるスコープの中では定数ではある値に関しては使わないでおいた方がいいかもしれない
* シャドーイングは可変にするのとは違う。let を使うことは実効的には新しい変数を生成していることになる。変数の名前が同じなだけで別の不変な値になっている
    ```rust
    // 代入
    let x = 5;
    x = 6;
    
    // シャドーイング
    let x = 5;
    let x = x + 1;
    ```
* 整数型には、8, 16, 32, 64 bit と OS のアーキテクチャ依存の size がある（それぞれ i: 正負, u: 正 がつく）
    * `i8` なら、-128 ~ 127 まで、`u8` なら 0 ~ 255 まで
* どの整数型・浮動小数点型を使うべきか？
    * 整数型の基準は `i32`（64bit system でもこれが最速）
    * `isize`, `usize` を使うのはなんらかのコレクションにアクセスするとき
    * 浮動小数点型の基準は `f64`（`f32` とほぼ同じスピードで精度が高い）
* 配列型は固定長であり、一度宣言されたら伸びも縮みもしない。ベクタ型は伸縮させることができる
    * 配列は、ヒープよりもスタックにデータのメモリを確保したいとき、または常に固定長の要素があることを確認したいときに有効（たとえば 1 年の月の名前）
    * どちらを使うべきか確証が持てないときはベクタ型を使うべき
* 式は文末に `;` を付けない。つけると文扱いになる
    * 式は評価されて値を返すもの、文は値を返さないもの 
* 所有権システムの規則の経験を積むと、より自然に安全かつ効率的なコードが書けるようになるとのこと
* スタックとヒープ
    * スタックは「積み重ねたお皿をとるとき、最後に積み重ねたお皿が最初に取得される」もの。データを追加することを push、取り除くことを pop。新しくデータを追加したり取り除くために場所を探す必要がないので高速
    * スタック上のデータはすべて既知の固定サイズでなければならない。なので、コンパイル時にサイズが分からないとか可変なデータはヒープに格納することができる。 データを置くときにはヒープ上に格納したいデータに十分な大きさの空領域を見つけ、そのアドレスを返す（これがポインタ）
    * ヒープデータを管理することが所有権の存在する理由である。どの部分のコードがどのヒープ上のデータを使用しているのか把握すること、ヒープ上の重複するデータを最小化すること、ヒープ上の未使用のデータを掃除することを所有権が解決する
* 所有権の規則
    * 各値は所有者と呼ばれる変数と対応
    * いかなるときも所有者は一つ
    * 所有者がスコープから外れたら値は破棄される
* `String` 型は、ヒープにメモリを確保するため、コンパイル時にサイズが不明なテキスト（標準入力など）も保持することができる
* ヒープデータの deep copy が必要な場合は `clone()` を使う。明示的に実行コストの高いことをしているのが伺える
* 関数に値は使わせるものの所有権はいらない場合は「参照」を使う
    * ref. [references.rc](./understanding-ownership/src/references.rc)
    * `&` が参照。`*` が参照外し
    * 関数の引数に参照をとることを借用
* 関数の戻り値を書くのが面倒だという理由で参照が存在していて、関数の中で値を変更したいというニーズはまだある。これを解消するために可変な参照がある、ということか
* スライスの `[start..end]` の `end` は、スライスの終端位置よりも 1 大きい値。hello_world の hello なら `[0..5]`、world なら `[6..]`。
* 文字列スライスの返り値の型は `&str`
* println! の {} の中に `:?` を使うと debug 出力ができる（`#[derive(Debug)]` という注釈が必要）。他には `:#?` がある
* 絶対パスを使うか相対パス（あるいは `super`）を使うか
    * 定義されたモジュールを使うコードと別々に動かす場合は絶対パス、一緒に動かす場合は相対パスというのが一つの判断材料になる
    * `super` は親モジュールを見に行くことができる。一緒に動かしそうな関係で使うのに向く
* `pub use` については [14.2](https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html#pub-use%E3%81%A7%E4%BE%BF%E5%88%A9%E3%81%AA%E5%85%AC%E9%96%8Bapi%E3%82%92%E3%82%A8%E3%82%AF%E3%82%B9%E3%83%9D%E3%83%BC%E3%83%88%E3%81%99%E3%82%8B) でも説明がある
    * ライブラリの利用者にとって、モジュールの内部構造が重要でない場合に、利用者にわかりやすい別の構造で公開することが可能になる
* args(): 引数のイテレータを返す。イテレータは一連の値を生成するもの。
    * イテレータとは、コレクションの種類や構造、実装によらず、先頭から順にアクセスするという操作を「最初の要素を指し示す」「次の要素へ行く」「前の操作に戻る」という抽象的な指示で記述ができるもの
    * collect(): イテレータが生成する要素全体を含むベクタなどのコレクションに変えるもの
    ```rust
    fn main() {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
    }
    ```
* `Box<T>` とは、値をボックス化、すなわちヒープ上に割り当てることができる（デフォルトではスタックに割り当てられるが）。ボックスとは正確にはヒープ上におかれた `T` の値へのスマートポインタである。ボックスがスコープを抜けるとヒープメモリが開放される

### 所有権
詳しくは https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html を読む。

これは動く。

ちなみに、この節を読んだあとだとなぜ x が無効化されていないのか矛盾してるように思うが、これは整数のようなコンパイル時に既知のサイズを持つ型はスタック上で保持されるためコピーするのも高速であり、無効化する理由がないから（スタックに保持されるかヒープに保持されるかという違いなのかー）。 \
また、整数のようなスタックに保持される型は `Copy` トレイトを実装していて、このトレイトがいる場合は代入後の古い変数が使用可能なままになる。逆に `Drop` トレイトを実装している場合は競合してコンパイルエラーになる
```rust
    let x = 5;
    let y = x;
    println!("x:{},y:{}", x,y); // x:5,y:5
```
上にならって次のように書くとこれは動かない。一見、s1 の値をコピーし s2 に束縛するということで ①s1, s2 がヒープデータのコピーは行わずに同じポインタ・長さ・許容量を持っているか（= shallow copy）、②ヒープデータのコピーも行っているか（=deep copy）のどっちかにより動作しそうに思う。 \
しかし、①のケースの場合、s1, s2 がスコープを抜けた際にそれぞれが自動でメモリを解放しようとすることによって二重解放エラーが起きることにつながるので困る。②の場合、ヒープ上のデータが大きい場合の実行時性能がとても悪くなるのでこれもイケてない。
```rust
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1:{},s2:{}", s1,s2);
```
①②のいずれでもないのだとしたらどうするのかというと、ポインタ・長さ・許容量のコピーに加えて s1 を無効化するという「ムーブ」という仕組みで実現する。

また、Rust では自動的にデータの deep copy が行われることは絶対にないので、s1 = s2 のようにコピーしてるような書き方をすることが実行時性能を悪化させることにはならない（以下の原文をこのように解釈）

> それ故に、あらゆる自動コピーは、実行時性能の観点で言うと、 悪くないと考えてよいことになります。

### モジュールシステム

* クレートにはバイナリクレートかライブラリクレートがある
* パッケージはある機能群を提供する 1 つ以上のクレート
  * クレートをどのようにビルドするかを説明するファイル Cargo.toml を持つ
  * 0 か 1 のライブラリクレートと 0 以上のバイナリクレートの、合わせて 1 つ以上のクレートを持たなければならない
  * cargo new によって作られるのはパッケージ
* モジュールは、クレート内のコードをグループ化する

## 疑問点

* `const` はグローバルスコープと書かれていてどこからでもアクセスできるのかと思ったが、「定義されたスコープ内でずっと有効です」と書かれていてどっちなんだろう（3 章）
* `String::from()` は可変にできるとのこと（4章）で可変な文字列リテラルを上書きするようなコードを書いたが最初の `s` が無効になっている。なんでだっけ？コンパイル時に初期化された値が使われていないことを知っているから？
    ```rust
        // テキストの例
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{}",s); // hello, world
    
        // 試したコード
        let mut s = "hello"; //  "value assigned to `s` is never read" という warning が出る
        s = "hoge";
        println!("{}",s); // hoge
    ```
* 「整数のようなスタックに保持される型は `Copy` トレイトを実装していて」とまとめたけど、スタックに保持される型はすべて `Copy` トレイトを実装できる？
* `dyn` とは？
    ```rust
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {}
    ```
