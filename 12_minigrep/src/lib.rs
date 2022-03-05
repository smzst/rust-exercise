use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    // &self をとらない関数が関連関数。:: で呼べるやつ
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // &args[1].clone だと &String 型になる
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// これだと、借用していた値を使って String（所有権を持つ）をフィールドを持つオブジェクトを作ろうとしており、所有権を奪っていることになってしまう
// 最良ではないが、clone() を使って値をコピーし所有権を持ったものを用意する
// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1];
//     let filename = &args[2];
//     Config{ query, filename }
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?; // ? をつけると呼び出し元が判断できるようにエラーをそのまま返す

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// search() に渡されるデータは、contents 引数に渡されるデータと同じ期間生きることを示す
// contents から、query を含むテキストの一部を返したいので contents が戻り値に関連していることを教えてあげている
// Vev<&str> ということで借りてきた値を返すことを示しているので、借りている主がいるはずである
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new(); // 伸び縮みできるのがベクタ。配列は固定長

    for line in contents.lines() { // lines() はイテレータを返す
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase(); // to_lowercase() は &str ではなく String を返す
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
