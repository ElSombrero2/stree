use std::io::stdin;

pub fn confirm(message: String, skip: bool) -> bool { 
    if skip {
        return true;
    }
    let mut prompt = String::new();
    println!("{message} [y/n] ");
    if stdin().read_line(&mut prompt).is_ok() {
        if prompt.to_lowercase().starts_with("y") {
            return true;
        }
    }
    return false;
}

