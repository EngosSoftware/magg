//! # Code of Conduct generator

/// Code of Conduct text.
const CODE_OF_CONDUCT: &str = include_str!("CODE_OF_CONDUCT.txt");

pub fn get_code_of_conduct(email: &str) -> String {
    assert_ne!("", email.trim()); //TODO Change this to regular error handling
    let mail = format!("[{0}](mailto:{0})", email.trim());
    CODE_OF_CONDUCT.replace("{{MAIL}}", &mail)
}
