# rust-exercise

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
