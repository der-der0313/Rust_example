use std::env;
use std::fs;
use std::process;


fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("解析参数出现问题:{}" ,err);
        process::exit(1);
    });
    
    let contents = fs::read_to_string(config.filename)
                                .expect("读取文件失败");
    println!("{}",contents);

}

struct Config{
    query:String,
    filename:String,

}
impl Config {
    fn new(args:&[String]) -> Result<Config,&str>{
        if args.len() < 3{
            return Err("参数不足");
        }
        Ok(Config{
            query:args[1].clone(),
            filename:args[2].clone(),
        })
    }
}


