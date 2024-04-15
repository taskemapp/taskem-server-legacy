use regex::{Error, Regex};
use tracing::error;

/// Returns ok if, and only if, the given string s matches the provided regex.
pub fn match_regex(r: &str, s: &str) -> Result<(), Error> {
    let regex = Regex::new(r)
        .map_err(|err| {
            error!(error = err.to_string(), "building regex");
        })
        .expect("Failed building regex");

    if !regex.is_match(s) {
        return Err(Error::Syntax(s.to_string()));
    }

    Ok(())
}
