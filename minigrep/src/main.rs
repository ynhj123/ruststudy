use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");
//
//     println!("With text:\n{}", contents);
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

