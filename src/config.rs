use std::path::Path;

pub struct ConfigEmails {
    /// Email address to be used in code of conduct file.
    pub code_of_conduct: String,
}

pub struct Config {
    pub emails: ConfigEmails,
}

pub fn load_config() -> Config {
    let home_dir = std::env::home_dir().expect("opening user's home directory failed");
    let content = std::fs::read_to_string(Path::new(&home_dir).join(".magg"))
        .expect("reading configuration file failed");
    let doc = diml::parse(&content).unwrap();
    let emails = doc.first_with_name("emails").unwrap();
    let code_of_conduct = emails
        .first_with_name("code-of-conduct")
        .unwrap()
        .content()
        .trim()
        .to_string();
    let emails = ConfigEmails { code_of_conduct };
    Config { emails }
}
