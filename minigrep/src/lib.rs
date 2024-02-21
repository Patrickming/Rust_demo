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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //当我们把访问一个没有权限的文件报错：Should have been able to read the file: Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        print!("{line}")
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    //这里的 lines 返回一个迭代器
    for line in contents.lines() {
        //Rust 的字符串还提供了 contains 方法，用于检查 line 是否包含待查询的 query。
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // 测试用例将在指定的内容中搜索 duct 字符串
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

