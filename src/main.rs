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
        //迄今为止，所有的输出信息，无论 debug 还是 error 类型，都是通过 println! 宏输出到终端的标准输出( stdout )，但是对于程序来说，错误信息更适合输出到标准错误输出(stderr)。
        //这样修改后，用户就可以选择将普通的日志类信息输出到日志文件 1，然后将错误信息输出到日志文件 2，甚至还可以输出到终端命令行。

        //eprintln!使得错误信息重定向到stderr里面，正确信息执行cargo run时会被成功打印
        eprintln!("Problem parsing arguments: {err}");
        //当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程
        //其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });
    // println!("Search for test {}",config.query);
    // println!("In file {}",config.file_path);
    // 代码执行时工作目录是根目录也就是minigrep目录下，而不是src目录
    // 用模式匹配处理返回的错误
    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error:{err}");
        process::exit(1);
    }
}