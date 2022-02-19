/*
   所有権と関数
*/
fn main_1() {
    let s = String::from("hello");
    takes_ownership(s); // 関数にムーブする
                        // println!("{}",s); // ここでは無効になる

    let x = 5;
    makes_copy(x); // 関数にムーブする
    println!("{}", x); // `Copy` を実装しているので使える
}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/*
   関数の戻り値について
*/
fn main_2() {
    let s1 = gives_ownership(); // 戻り値を s1 にムーブ
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 を関数にムーブし、戻り値は s3 にムーブ
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
