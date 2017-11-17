pub mod filesys;
pub mod parse;
pub mod search;

use std::error::Error;

pub fn run(config: parse::Config) -> Result<(), Box<Error>> {
    let text = filesys::read_file(&config.path[..])?;
    if config.sensitive {
        search::search_sensitive(&config.query[..], &text[..]);
    } else {
        search::search_insensitive(&config.query[..], &text[..]);
    }
    Ok(())
}

#[cfg(test)]
mod test;