use std::error::Error;
use std::{fs, env};
pub struct Config {
    #[warn(dead_code)]
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool,
}

impl Config {
    // #[warn(dead_code)]
    // fn new(args:&[String])->Config{
    //     let query = &args[1];
    //     let file_path = &args[2];
    //     Config { query, file_path }
    // }

    // 有一点需要额外注意下，从代码惯例的角度出发，new 往往不会失败，毕竟新建一个实例没道理失败
    // 所以改成build会比较合适
    pub fn build(mut args: std::env::Args) -> Result<Config,&'static str> {
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        //忽略大小写 any返回的是bool正好符合
        let ignore_case = env::var("IGNORE_CASE").map_or_else(
        |_| args.any(|arg| {arg.to_lowercase() == "-i" || arg.to_lowercase() == "--ignore-case"}), 
        |env_value| env_value == "1" || env_value.to_lowercase() == "true");
        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    //增加错误处理
    let contents = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    }else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query,content));
    }
    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, content));
    }
}
// 找到content中存在query的行，并保存所在行
pub fn search<'a>(query:&str,content:&'a str) -> Vec<&'a str> {
    content.lines()
    .filter(|line| line.contains(query))
    .collect()
}
//大小写不敏感查询
pub fn search_case_insensitive<'a>(query:&str,content:&'a str) -> Vec<&'a str> {
    content.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}