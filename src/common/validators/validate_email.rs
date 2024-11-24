use regex::Regex;

pub fn validate_email(email: String) -> Result<String, String> {
    let email = email;
    let email_regex = Regex::new(
        r"^([a-z0-9-_+]([a-z0-9-_+.]*[a-z0-9-_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    if email_regex.is_match(&email) {
        Ok(String::from("success"))
    } else {
        Err(String::from("error to validate email"))
    }
}
