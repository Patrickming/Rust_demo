
use std::env;
use std::fs;
fn main() {
    //读取并分析传入的命令行参数，然后通过 collect 方法输出一个集合类型 Vector。
    let args: Vec<String>= env::args().collect();
    // dbg!(args);

    //增加读取参数 格式cargo run -- query file_path
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
