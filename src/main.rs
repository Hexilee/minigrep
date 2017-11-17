#[macro_use]
extern crate clap;
extern crate minigrep;
use std::process;
use minigrep::*;
use minigrep::parse::*;

fn main() {
    let matches = clap_app!(minigrep => 
        (version: "1.0")
        (author: "Hexi Lee. <hexiisme@gmail.com>")
        (about: "\n\
    Search for PATTERN in PATH.
Like grep or ack, written in rust
    ")
        (@arg QUERY: +required "The query pattern")
        (@arg FILE: +required "The file to be searched")
        (@arg sensitive: -s ... "If sensitive (default not)")
    ).get_matches();

    
    // let args: Vec<String> = env::args().collect();
    let config = match Config::new(matches) {
        Ok(c) => c,
        Err(s) => {
            println!("Wrong usage!\n {}", s);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        println!("Application error: {:?}", e);
        process::exit(1);
    }
}