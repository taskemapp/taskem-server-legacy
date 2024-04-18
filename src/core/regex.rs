use regex::{Error, Regex};

use crate::domain::constants::EMAIL;

#[derive(Default)]
pub struct CachedRegexValidator {
    email: Option<Regex>,
}

impl CachedRegexValidator {
    pub fn compile_all(&mut self) {
        self.email = Some(Regex::new(EMAIL).unwrap())
    }

    pub fn check_email(&self, email: &str) -> Result<(), Error> {
        match &self.email {
            None => Err(Error::Syntax(String::from("Email pattern not init."))),
            Some(regex) => {
                if !regex.is_match(email) {
                    Err(Error::Syntax(email.to_string()))
                } else {
                    Ok(())
                }
            }
        }
    }
}
