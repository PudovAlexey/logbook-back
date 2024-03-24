use regex::Regex;

pub fn validate_email(email: String) -> Result<String, String> {
    let email = email;
    let email_regex = Regex::new(r"^([a-z0-9-_+]([a-z0-9-_+.]*[a-z0-9-_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    if email_regex.is_match(&email) {
        Ok(String::from("success"))
    } else {
        Err(String::from("error to validate email"))
    }
}

pub fn validate_password(password: String) -> Result<String, String> {

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_whitespace = false;
    let mut has_chars = 0;
    for c in password.chars() {
        has_chars += 1;
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_digit(10) {
            has_digit = true;
        } else if c.is_whitespace() {
            has_whitespace = true;
        }
    }
   let matching = has_chars >= 8 && has_uppercase && has_lowercase && has_digit && !has_whitespace;

   if matching {
    Ok(String::from("success"))
   } else {
    Err(String::from("error to verify password"))
   }

}