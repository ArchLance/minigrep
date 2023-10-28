use std::error::Error;
use std::fs;
pub struct Config<'a> {
    #[warn(dead_code)]
    pub query : &'a String,
    pub file_path : &'a String,
}

impl <'a> Config<'a> {
    // #[warn(dead_code)]
    // fn new(args:&[String])->Config{
    //     let query = &args[1];
    //     let file_path = &args[2];
    //     Config { query, file_path }
    // }

    // 有一点需要额外注意下，从代码惯例的角度出发，new 往往不会失败，毕竟新建一个实例没道理失败
    // 所以改成build会比较合适
    pub fn build(args:&[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        
        Ok(Config { query, file_path})
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    //增加错误处理
    let contents = fs::read_to_string(config.file_path)?;
    println!("with content:\n{contents}");
    Ok(())
}