use core::fmt;
use std::error;

#[allow(nonstandard_style, unused)]
#[derive(Debug, Clone)]
pub struct SamError
where
Self: error::Error + Send {
    message: String,
}

impl fmt::Display for SamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for SamError {}

#[cfg(test)]
mod tests {
    use super::SamError;

    #[test]
    fn error_message() {
        let result: Result<i32, SamError> = Result::Err(SamError {
            message: "unknown error.".to_string(),
        });
        match result {
            Ok(_) => panic!("ERROR."),
            Err(e) => assert_eq!(e.to_string(), "unknown error.".to_string()),
        }
    }
}
