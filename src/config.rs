use std::path::Path;

pub struct ConfigEmails {
    /// Email address to be used in code of conduct file.
    pub code_of_conduct: String,
}

pub struct ConfigLicenses {
    /// Copyright owner to be placed in licenses.
    pub copyright_owner: String,
}

pub struct ConfigCratesIo {
    /// Account id on crates.io.
    pub id: String,
}

pub struct Config {
    pub licenses: ConfigLicenses,
    pub emails: ConfigEmails,
    pub crates_io: ConfigCratesIo,
}

pub fn load_config() -> Config {
    let home_dir = std::env::home_dir().expect("opening user's home directory failed");
    let content = std::fs::read_to_string(Path::new(&home_dir).join(".magg"))
        .expect("reading configuration file failed");
    let doc = diml::parse(&content).unwrap();

    let licenses = doc.first_with_name("licenses").unwrap();
    let copyright_owner = licenses
        .first_with_name("copyrightOwner")
        .unwrap()
        .content()
        .trim()
        .to_string();

    let emails = doc.first_with_name("emails").unwrap();
    let code_of_conduct = emails
        .first_with_name("codeOfConduct")
        .unwrap()
        .content()
        .trim()
        .to_string();

    let crates_io = doc.first_with_name("crates.io").unwrap();
    let id = crates_io
        .first_with_name("id")
        .unwrap()
        .content()
        .trim()
        .to_string();

    let licenses = ConfigLicenses { copyright_owner };
    let emails = ConfigEmails { code_of_conduct };
    let crates_io = ConfigCratesIo { id };

    Config {
        licenses,
        emails,
        crates_io,
    }
}
