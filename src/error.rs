
use std::result;
use std::error;
use std::fmt;
use pest;
use pest::Span;

use crate::Rule;

#[derive(Debug)]
pub enum Error {
    Parsing(pest::error::Error<Rule>),
    Code(String, pest::error::Error<()>),
    Formatting(fmt::Error),
}

pub type Result<A> = result::Result<A, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Parsing(e) => <pest::error::Error<Rule> as error::Error>::description(&e),
            Error::Code(_, e) => <pest::error::Error<()> as error::Error>::description(&e),
            Error::Formatting(..) => "error writing to buffer",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res_str = match self {
            Error::Parsing(e) => format!("parsing error:\n{}", e),
            Error::Code(name, e) => format!("{}\n{}", name, e),
            Error::Formatting(e) => format!("{}", e),
        };
        // Weird bug with pest
        res_str = res_str.replace("␊", "");
        write!(f, "{}", res_str)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Error::Parsing(error)
    }
}

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Error::Formatting(error)
    }
}

pub fn error_from_message_span(err_type: String, message: String, span: Span) -> Error {
    Error::Code(
        err_type,
        pest::error::Error::<()>::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: message
            }, span
        )
    )
}
