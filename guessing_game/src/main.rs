use std::cmp::Ordering;
use std::io;

// なぜ rand::thread_rng じゃないのか
// `Rng` トレイトは乱数生成器が実装するメソッドを定義していて、このトレイトがスコープにないとメソッドを使用できないため
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // シャドーイングはある値を別の型に変換したいときによく使われる guess:String → guess:u32
        // trim(): 両端の空白を削除（\n が入るのを除きたい）
        // let guess: u32 か parse::<u32>() どっちかでも推論は通せる
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
