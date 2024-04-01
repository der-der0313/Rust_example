use std::env;
use std::process;
use minigrep::Config;



fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("解析参数出现问题:{}" ,err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("错误:{}",e);
        process::exit(1)
    }

}


