pub mod unify;
use unify::solve_unifier;
pub mod solve;
pub mod parse;
pub mod builtins;

use std::fmt::Write;

use std::collections::HashMap;

pub type Unifier = HashMap<UnknownContents, Term>;


pub fn fmt_unifier(unif: &Unifier) -> String {
    let mut res = String::new();
    let mut first = true;
    for (key, val) in unif {
        if !key.name.starts_with("<Tmp>") {
            if first {
                first = false;
            } else {
                write!(&mut res, ", ").expect("formatting error");
            }
            write!(&mut res, "{} = {:?}", &key.name, &val).expect("formatting error");
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

// impl Rules {
//     pub fn mangle_names(&mut self) {
//         for rule in self.contents.iter_mut() {
//             rule.mangle_names();
//         }
//     }
// }

impl Rule {
    pub fn collect_names(&self) -> Vec<UnknownContents> {
        let mut res = vec![];
        for item in self.gives.args.iter() {
            res.append(&mut item.collect_names());
        }
        for term in self.requires.goals.iter() {
            res.append(&mut term.collect_names());
        }
        res
    }
//     pub fn mangle_names(&mut self) {
//         let mut rng = thread_rng();
//         let names = self.collect_names();
//         let mut name_subs: Unifier =
//             Unifier::new();
//         for name in names {
//             let new_name: String = format!("_<{}>", iter::repeat(())
//                 .map(|()| rng.sample(Alphanumeric))
//                 .filter(|c| !c.is_digit(10))
//                 .take(6)
//                 .collect::<String>());
//             name_subs.insert(name.clone(), Term::Unknown(new_name));
//         }
//         for item in self.gives.args.iter_mut() {
//             item.substitute_all(&name_subs);
//         }
//         for term in self.requires.goals.iter_mut() {
//             term.substitute_all(&name_subs);
//         }
//     }
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
    Number(f32),
    List(ListTerm),
    Compound(CompoundTerm),
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

impl Term {
    pub fn collect_names(&self) -> Vec<UnknownContents> {
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
    Unknown(UnknownContents),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundTerm {
    pub name: String,
    pub args: Vec<Term>,
}

