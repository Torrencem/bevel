pub mod unify;
pub mod solve;
pub mod parse;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::iter;
use std::fmt::Write;

use std::collections::HashMap;

pub type Unifier = HashMap<String, Term>;

pub fn fmt_unifier(unif: &Unifier) -> String {
    let mut res = String::new();
    let mut first = true;
    for (key, val) in unif {
        if !key.starts_with("<Tmp>") {
            if first {
                first = false;
            } else {
                write!(&mut res, ", ").expect("formattign error");
            }
            write!(&mut res, "{} = {:?}", &key, &val).expect("formatting error");
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

impl Rules {
    pub fn mangle_names(&mut self) {
        for rule in self.contents.iter_mut() {
            rule.mangle_names();
        }
    }
}

impl Rule {
    pub fn collect_names(&self) -> Vec<String> {
        let mut res = vec![];
        for item in self.gives.args.iter() {
            res.append(&mut item.collect_names());
        }
        for term in self.requires.goals.iter() {
            res.append(&mut term.collect_names());
        }
        res
    }

    pub fn mangle_names(&mut self) {
        let mut rng = thread_rng();
        let names = self.collect_names();
        let mut name_subs: Unifier =
            Unifier::new();
        for name in names {
            let new_name: String = format!("_<{}>", iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .filter(|c| !c.is_digit(10))
                .take(6)
                .collect::<String>());
            name_subs.insert(name.clone(), Term::Unknown(new_name));
        }
        for item in self.gives.args.iter_mut() {
            item.substitute_all(&name_subs);
        }
        for term in self.requires.goals.iter_mut() {
            term.substitute_all(&name_subs);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub goals: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Unknown(String),
    Atom(String),
    Number(f32),
    List(ListTerm),
    Compound(CompoundTerm),
}

impl Term {
    pub fn collect_names(&self) -> Vec<String> {
        match self {
            Term::Unknown(s) => {
                vec![s.clone()]
            },
            Term::Atom(_) => vec![],
            Term::Number(_) => vec![],
            Term::List(lterm) => {
                let mut result = vec![];
                for term in lterm.front.iter() {
                    result.append(&mut term.collect_names());
                }
                if let ListTail::Unknown(s) = &lterm.tail {
                    result.push(s.clone());
                }
                result
            },
            Term::Compound(cterm) => {
                let mut result = vec![];
                for term in cterm.args.iter() {
                    result.append(&mut term.collect_names());
                }
                result
            }
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
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundTerm {
    pub name: String,
    pub args: Vec<Term>,
}

