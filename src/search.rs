pub fn search_sensitive<'a>(query: &str, cotent: &'a str){
    for (n, line) in cotent.lines().enumerate() {
        if line.contains(query) {
            println!("line {}: {}", n, line);
        }
    }
}

pub fn search_insensitive<'a>(query: &str, cotent: &'a str){
    let query = query.to_lowercase();
    for (n, line) in cotent.lines().enumerate() {
        if line.to_lowercase().contains(&query[..]) {
            println!("line {}: {}", n, line);
        }
    }
}