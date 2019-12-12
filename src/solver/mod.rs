mod unify;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Unknown(String),
    Atom(String),
    Number(f32),
    List(ListTerm),
    Compound(CompoundTerm),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListTerm {
    front: Vec<Term>,
    tail: ListTail,
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
    name: String,
    args: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    goals: Vec<CompoundTerm>
}
