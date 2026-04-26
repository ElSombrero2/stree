use std::io::stdin;

pub fn confirm(message: String, skip: bool) -> bool { 
    if skip { return true; }
    let mut prompt = String::new();
    println!("{message} [y/n] ");
    stdin().read_line(&mut prompt).is_ok() && prompt.to_lowercase().starts_with("y")
}

