use std::env;
use std::process;
use minigrep::Config;
fn main() {
    //读取并分析传入的命令行参数，然后通过 collect 方法输出一个集合类型 Vector。
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    //增加读取参数 格式cargo run -- query file_path
    /*如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；
    如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理 */
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        //不再panic!让程序崩溃，而是终结进程
        process::exit(1); //非0数
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

   if let Err(e) = minigrep::run(config){
       println!("Application error: {e}");
       process::exit(1);
   } ;
}
