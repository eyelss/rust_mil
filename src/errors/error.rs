use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorSpan {
  errors: Vec<Box<dyn Error>>
}

impl fmt::Display for ErrorSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed with {} length span of errors:", self.errors.len())?;

        for error in &self.errors {
            write!(f, "\n- {}", error)?;
        }

        Ok(())
    }
}

impl Error for ErrorSpan {}