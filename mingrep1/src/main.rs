use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In File{}", config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    };

} 
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    Ok(());

    println!("The contents of {}", contents);
}
struct Config{
    query: String,
    filename: String
}

fn parse_config(args: &[String]) ->(&str, &str) {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{query, filename};
}
impl Config{
    fn new(args: &[String]) ->Result<Congif,&str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        OK(Config{query, filename});
}
}