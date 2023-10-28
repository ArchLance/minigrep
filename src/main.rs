use std::env;
use std::fs;
use std::process;
fn main() {
    //收集命令行输入参数
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Search for test {}",config.query);
    println!("In file {}",config.file_path);
    // 代码执行时工作目录是根目录也就是minigrep目录下，而不是src目录
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("with content:\n{contents}");
}
struct Config<'a> {
    #[warn(dead_code)]
    query : &'a String,
    file_path : &'a String,
}

impl <'a> Config<'a> {
    // #[warn(dead_code)]
    // fn new(args:&[String])->Config{
    //     let query = &args[1];
    //     let file_path = &args[2];
    //     Config { query, file_path }
    // }
    fn build(args:&[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        
        Ok(Config { query, file_path})
    }
}