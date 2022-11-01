use std::error::Error;
use std::fmt;

type NomError<I> = nom::error::Error<I>;
pub type WrappedNomError<'a> = nom::Err<NomError<&'a str>>;

#[derive(Debug)]
pub struct InvalidFormulaError {
    pub failed_tail: String,
}

impl InvalidFormulaError {
    pub fn new(failed_tail: String) -> Self {
        Self { failed_tail }
    }
}

impl fmt::Display for InvalidFormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed formula tail={}", self.failed_tail)
    }
}

impl Error for InvalidFormulaError {}

impl From<WrappedNomError<'_>> for InvalidFormulaError {
    fn from(err: WrappedNomError<'_>) -> Self {
        //let message =  format!("Could not parse whole expression. Stopped at tail={}", err.input)
        let failed_tail = match err {
            WrappedNomError::Incomplete(_) => "not enough data".to_string(),
            WrappedNomError::Error(inner) => inner.input.to_string(),
            WrappedNomError::Failure(inner) => {
                format!("Critical inner error: {}", inner)
            }
        };
        Self { failed_tail }
    }
}

#[derive(Debug)]
pub struct BadOperatorError {
    message: String,
}

impl BadOperatorError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for BadOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BadOperatorError {}
