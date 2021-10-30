use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // unwrap_or_else接收到一个Err值时，会调用传入的那个闭包函数
    // let config = Config::new(&args).unwrap_or_else(|err| {
    // env::args() 返回一个iterator
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 标准错误输出
        eprintln!("Problem parsing arguments: {}", err);
        // process.exit会立刻停止程序并返回传给它的数字，非0值是错误状态退出的约定俗成的值
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
