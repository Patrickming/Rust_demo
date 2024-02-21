use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //当我们把访问一个没有权限的文件报错：Should have been able to read the file: Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
    Ok(())
}
