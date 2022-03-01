/*
    相対パスと絶対パスについて
 */
mod front_of_house { // 同じレベルのモジュールは見れる
    pub mod hosting { // 子モジュールはデフォルトでは private なので public に
        pub fn add_to_waitlist() {} // これも同じく
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // 絶対パス
    front_of_house::hosting::add_to_waitlist(); // 相対パス
}

/*
    super について
 */
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

/*
    モジュールで構造体を扱う
 */
mod back_of_house{
    pub struct Breakfast{ // 構造体自体を公開する
        pub toast: String, // そのうちの特定のフィールドを公開する
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // ちなみにこの関数へは、back_of_house::hello() となる
    // pub fn hello(){}
}

pub fn eat_at_restaurant(){
    // meal = Breakfast{ toast: "Rye", seasonal_fruit: "peaches" }
    let mut meal = back_of_house::Breakfast::summer("Rye"); // mod 内の構造体とか関数には :: でアクセスして、. を使うのは構造体のフィールドなど

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // println!("{}", meal.seasonal_fruit); // private field なのでアクセスできない
}

/*
    モジュールで構造体を扱う
 */

mod back_of_house{
    pub enum Appetizer{
        Soup, Salad, // enum のヴァリアントは全部 public
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

/*
    use を使ってパスをスコープに持ち込む
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // 絶対パスでの指定
use self::front_of_house::hosting; // 相対パスでの指定

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // hosting:: とすることで他所からの呼び出しであることを表現するのが慣習
}

mod aaaaa {
    use super::front_of_house::hosting; // 親を辿ることもできる
    fn bbbbbbb() {
        hosting::add_to_waitlist();
    }
}
