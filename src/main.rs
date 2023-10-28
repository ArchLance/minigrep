use std::env;

fn main() {
    //收集命令行输入参数
    let args:Vec<String> = env::args().collect();
    dbg!(args);
}
