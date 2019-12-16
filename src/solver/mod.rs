pub mod unify;
use unify::solve_unifier;
pub mod solve;
pub mod parse;
pub mod builtins;
pub mod optimize;

use std::fmt;

use num_rational::Rational32;

use std::fmt::Write;

use std::collections::HashMap;

pub type Unifier = HashMap<UnknownContents, Term>;

pub fn fmt_unifier(unif: &Unifier) -> String {
    let mut res = String::new();
    let mut first = true;
    for (key, val) in unif {
        if !key.name.starts_with("<Free>") {
            if first {
                first = false;
            } else {
                write!(&mut res, ", ").expect("formatting error");
            }
            write!(&mut res, "{} = {}", &key.name, &val).expect("formatting error");
        }
    }
    res
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rules {
    pub contents: Vec<Rule>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub gives: CompoundTerm,
    pub requires: Query,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub goals: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownContents {
    pub name: String,
    pub frame_id: u32
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Unknown(UnknownContents),
    Atom(String),
    Number(Rational32),
    List(ListTerm),
    Compound(CompoundTerm),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Unknown(UnknownContents{
                name: s,
                frame_id: _,
            }) => write!(f, "{}", s),
            Term::Atom(s) => {
                write!(f, "{}", s[1..].to_string())
            },
            Term::Number(n) => {
                write!(f, "{}", n)
            },
            Term::List(lterm) => {
                write!(f, "{}", lterm)
            },
            Term::Compound(cterm) => {
                write!(f, "{}", cterm)
            },
        }
    }
}

impl fmt::Display for ListTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for term in self.front.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", term)?;
        }
        match &self.tail {
            ListTail::End => write!(f, "]"),
            ListTail::Unknown(UnknownContents{
                name: s,
                frame_id: _,
            }) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for CompoundTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "(")?;
        let mut first = true;
        for term in self.args.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", term)?;
        }
        write!(f, ")")
    }
}

impl Term {
    pub fn set_new_frame_id(&mut self, frame_id: u32) {
        match self {
            Term::Unknown(contents) => {
                contents.frame_id = frame_id;
            },
            Term::Atom(..) => {},
            Term::Number(..) => {},
            Term::List(lterm) => {
                for term in lterm.front.iter_mut() {
                    term.set_new_frame_id(frame_id);
                }
                if let ListTail::Unknown(contents) = &mut lterm.tail {
                    contents.frame_id = frame_id;
                }
            },
            Term::Compound(cterm) => {
                for term in cterm.args.iter_mut() {
                    term.set_new_frame_id(frame_id);
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListTerm {
    pub front: Vec<Term>,
    pub tail: ListTail,
}

impl ListTerm {
    pub fn empty() -> ListTerm {
        ListTerm {
            front: vec![],
            tail: ListTail::End
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListTail {
    End,
    Unknown(UnknownContents),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundTerm {
    pub name: String,
    pub args: Vec<Term>,
}

