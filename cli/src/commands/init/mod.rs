use std::fs; 

pub fn init (filename: String) {
    const EMPTY_CONFIG: &str = r#"
endpoint = ""
credential = { access_key = "", secret_key = "" }
force_path_style = true
region = ""
"#;

    if let Ok(exists) = fs::exists(&filename) {
        if !exists {
            fs::write(&filename, EMPTY_CONFIG).unwrap();
            let gitignore_path = "./.gitignore";
            if let Ok(mut gitignore) = fs::read_to_string(gitignore_path) {
                gitignore.push_str(&filename.replace("./", ""));
                fs::write(gitignore_path, gitignore).unwrap();
            }
            println!("✅ Project initialized!");
        } else {
            println!("⚠️ You have already a config file with that same name");
        }
    }
}
