use std::env;
use std::process;

use minigrep::Config;
fn main() {
    //收集命令行输入参数
    let args:Vec<String> = env::args().collect();
    //unwrap_or_else 是定义在 Result<T,E> 上的常用方法，
    //如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；
    //如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        //当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程
        //其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });
    println!("Search for test {}",config.query);
    println!("In file {}",config.file_path);
    // 代码执行时工作目录是根目录也就是minigrep目录下，而不是src目录
    // 用模式匹配处理返回的错误
    if let Err(e) = minigrep::run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}