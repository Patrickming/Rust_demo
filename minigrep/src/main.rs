use std::env;
use std::error::Error;
use std::fs;
use std::process;
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

   if let Err(e) =  run(config){
       println!("Application error: {e}");
       process::exit(1);
   } ;
}
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //当我们把访问一个没有权限的文件报错：Should have been able to read the file: Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            //query + filepath +  "target\\debug\\minigrep.exe",
            return Err("not enough arguments!");
        }
        //这里Config结构体string需要拥有所有权
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
