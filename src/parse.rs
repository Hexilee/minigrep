extern crate clap;
use self::clap::ArgMatches;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Self, &'static str> {
        let query = match matches.value_of("QUERY") {
            Some(s) => s.to_string(),
            None => return Err("usage: minigrep [search string] [file]"),
        };

        let path = match matches.value_of("FILE") {
            Some(s) => s.to_string(),
            None => return Err("usage: minigrep [search string] [file]"),
        };

        match matches.occurrences_of("sensitive") {
            0 => Ok(Config { query, path, sensitive: false}),
            1 | _ => Ok(Config { query, path, sensitive: true})
        }
    }
}