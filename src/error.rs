
use annotate_snippets::snippet::Snippet;
use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;

use std::result;
use std::error;
use std::fmt;
use pest;

use crate::span::Span;
use crate::Rule;

#[derive(Debug)]
pub enum Error {
    Parsing(pest::error::Error<Rule>),
    Code(Snippet),
    Formatting(fmt::Error),
}

pub type Result<A> = result::Result<A, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Parsing(e) => <pest::error::Error<Rule> as error::Error>::description(&e),
            Error::Code(s) => {
                match &s.title {
                    None => "code error",
                    Some(title) => {
                        match &title.label {
                            None => "code error",
                            Some(s) => {
                                s.as_ref()
                            }
                        }
                    }
                }
            },
            Error::Formatting(..) => "error writing to buffer",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res_str = match self {
            Error::Parsing(e) => format!("parsing error:\n{}", e),
            Error::Code(s) => {
                let dlf = DisplayListFormatter::new(true, false);
                
                let dl: DisplayList = s.clone().into();

                dlf.format(&dl)
            },
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

pub fn union_spans<'p>(span1: &Span<'p>, span2: &Span<'p>) -> Span<'p> {
    Span {
        input: span1.input,
        start: span1.start,
        end: span2.end,
    }
}

// pub fn to_line_start(span: &Span) -> Span {
//
// }
