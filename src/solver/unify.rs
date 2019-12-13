
use std::cmp::min;
use crate::solver::*;
use Term::*;

use std::collections::{HashMap, VecDeque};

pub type Goal = Vec<(Term, Term)>;

pub fn solve_unifier(unif: &Unifier) -> Unifier {
    let mut res = Unifier::new();

    for (key, val) in unif.iter() {
        let mut result = Unknown(key.clone());
        let mut curr = key;
        while let Some(val) = unif.get(curr) {
            result = val.clone();
            if let Unknown(x) = val {
                curr = x;
            } else {
                break;
            }
        }
        res.insert(key.clone(), result);
    }
    res
}

pub fn compute_most_gen_unifier(goal: Goal) -> Option<Unifier> {
    // https://stackoverflow.com/a/49114348
    let mut equations: VecDeque<(bool, Term, Term)> = 
        goal.into_iter()
        .map(|(t1, t2)| (false, t1, t2))
        .collect();
    loop {
        let equality = equations.pop_front()?;
        if equality.0 {
            equations.push_back(equality);
            continue;
        }
        match (equality.1, equality.2) {
            (Unknown(x), Unknown(y)) => {
                if x == y {} // trivial
                else {
                    // elimination
                    equations.push_back((true, Unknown(x.clone()), Unknown(y.clone())));
                    // apply substitution to all other problems
                    for (done, lhs, rhs) in equations.iter_mut() {
                        if *done { continue; }
                        lhs.simple_substitution(&x, &Unknown(y.clone()))?;
                        rhs.simple_substitution(&x, &Unknown(y.clone()))?;
                    }
                }
            },
            (yterm, Unknown(x)) => {
                // orient
                equations.push_back((false, Unknown(x.clone()), yterm.clone()));
                continue;
            },
            (Unknown(x), yterm) => {
                // elimination
                equations.push_back((true, Unknown(x.clone()), yterm.clone()));
                // apply substitution to all other problems
                for (done, lhs, rhs) in equations.iter_mut() {
                    if *done { continue; }
                    lhs.simple_substitution(&x, &yterm)?;
                    rhs.simple_substitution(&x, &yterm)?;
                }
            }, // Neither variable is unknown, either decomposition, clash, or trivial
            (Atom(x), Atom(y)) if x == y => { }, // trivial
            (Atom(_), _) => { return None; }, // clash
            (Number(x), Number(y)) if x == y => { },
            (Number(_), _) => { return None; },
            (List(lterm), List(lterm2)) => {
                // Attempt decomposition
                let minlen = min(lterm.front.len(), lterm2.front.len());
                for indx in 0..minlen {
                    equations.push_back((false, lterm.front[indx].clone(), lterm2.front[indx].clone()));
                }
                if lterm.front.len() == lterm2.front.len() {
                    match (&lterm.tail, &lterm2.tail) {
                        (ListTail::End, ListTail::End) => {},
                        (ListTail::Unknown(s), ListTail::End) => {
                            equations.push_back((false, Unknown(s.clone()), List(ListTerm::empty())));
                        },
                        (ListTail::End, ListTail::Unknown(s)) => {
                            equations.push_back((false, Unknown(s.clone()), List(ListTerm::empty())));
                        },
                        (ListTail::Unknown(a), ListTail::Unknown(b)) => {
                            equations.push_back((false, Unknown(a.clone()), Unknown(b.clone())));
                        }
                    }
                }
                if lterm.front.len() < lterm2.front.len() {
                    match &lterm.tail {
                        ListTail::End => {
                            // clash
                            return None;
                        },
                        ListTail::Unknown(s) => {
                            // Collect the rest of the elements
                            // of lterm2
                            let rest_vec: Vec<Term> = 
                                lterm2.front[minlen..]
                                .iter().clone()
                                .map(|x| x.clone()).collect();
                            let as_lterm = ListTerm {
                                front: rest_vec,
                                tail: ListTail::End,
                            };
                            equations.push_back((false, Unknown(s.clone()), List(as_lterm)));
                        },
                    }
                }
                if lterm.front.len() > lterm2.front.len() {
                    match &lterm2.tail {
                        ListTail::End => {
                            // clash
                            return None;
                        },
                        ListTail::Unknown(s) => {
                            // Collect the rest of the elements
                            // of lterm2
                            let rest_vec: Vec<Term> = 
                                lterm.front[minlen..]
                                .iter().clone()
                                .map(|x| x.clone()).collect();
                            let as_lterm = ListTerm {
                                front: rest_vec,
                                tail: ListTail::End,
                            };
                            equations.push_back((false, Unknown(s.clone()), List(as_lterm)));
                        },
                    }
                }
                continue;
            },
            (List(_), _) => { return None; },
            (Compound(cterm), Compound(cterm2)) => {
                if cterm.name != cterm2.name {
                    return None;
                }
                if cterm.args.len() != cterm2.args.len() {
                    return None;
                }
                for indx in 0..cterm.args.len() {
                    equations.push_back((false, cterm.args[indx].clone(), cterm2.args[indx].clone()));
                }
                continue;
            },
            (Compound(_), _) => { return None; },
        }
        let mut done = true;
        // Since we haven't 'continue'd, that
        // indicates we should check if we're done
        for (f, lhs, _) in equations.iter() {
            if !f {
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }

    let mut res: HashMap<String, Term> = HashMap::with_capacity(equations.len());
    for (_, lhs, rhs) in equations {
        if let Unknown(s) = lhs {
            res.insert(s, rhs);
        }
    }
    Some(res)
}

impl Term {
    // Returns None if trying to substitute like this example:
    // [H | T] with subs T/100
    // can't work because T *must* be a list
    pub fn simple_substitution(&mut self, unknown: &String, subs: &Term) -> Option<()> {
        match self {
            Unknown(s) if s == unknown => {
                *self = subs.clone();
            },
            Unknown(_) | Atom(_) | Number(_) => { },
            List(lterm) => {
                if let ListTail::Unknown(s) = &lterm.tail {
                    if s == unknown {
                        if let List(lterm2) = subs {
                            lterm.front.append(&mut lterm2.front.clone());
                            lterm.tail = lterm2.tail.clone();
                        } else {
                            return None;
                        }
                    }
                }
                for item in lterm.front.iter_mut() {
                    item.simple_substitution(unknown, subs);
                }
            },
            Compound(cterm) => {
                for item in cterm.args.iter_mut() {
                    item.simple_substitution(unknown, subs);
                }
            },
        }
        Some(())
    }

    pub fn substitute_all(&mut self, un: &Unifier) -> Option<()> {
        for (unknown, subs) in un.iter() {
            self.simple_substitution(unknown, subs)?;
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unify() {
        let term1 = Compound(CompoundTerm {
            name: "member".to_string(),
            args: vec![
                Unknown("Z".to_string()),
                List(ListTerm {
                    front: vec![
                        Atom("a".to_string()),
                        Atom("b".to_string())
                    ],
                    tail: ListTail::End
                }),
            ]
        });
        let term2 = Compound(CompoundTerm {
            name: "member".to_string(),
            args: vec![
                Unknown("X".to_string()),
                List(ListTerm {
                    front: vec![
                        Unknown("X".to_string()),
                    ],
                    tail: ListTail::Unknown("Y".to_string())
                }),
            ]
        });
        
        let goal = vec![(term1, term2)];
        let unifier = compute_most_gen_unifier(goal);
        println!("{:?}", &unifier);
        if let Some(unifier) = &unifier {
            println!("{:?}", solve_unifier(&unifier));
        }
    }
}
