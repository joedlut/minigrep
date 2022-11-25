use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("problems parsing config: {}",err);
        process::exit(1);
    });
    //println!("{}",config.query);
    //println!("{}",config.file_path);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error:{}",e);
        process::exit(1);
    }

}
