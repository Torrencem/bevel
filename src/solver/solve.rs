
use crate::solver::*;
use crate::solver::unify::*;

use std::collections::{HashMap, VecDeque};

pub fn solve(facts: &Rules, query: Query) -> Option<Unifier> {
    let mut fact_indx: usize = 0;
    let mut master: Unifier = Unifier::new();
    let mut curr_query: Query = query;
    let mut new_query: Query = Query { goals: vec![] };
    // A stack of querys, states, and fact indices
    let mut choice_points: Vec<(Unifier, Query, usize)> = Vec::new();
    loop {
        match curr_query.goals.get(0) {
            None => return Some(solve_unifier(&master)),
            Some(goal) => {
                // Find a clause that matches
                // the current goal
                let mut nomatching = true;
                for clause in facts.contents[fact_indx..].iter() {
                    let unification =
                        compute_most_gen_unifier(vec![(goal.clone(), Term::Compound(clause.gives.clone()))]);
                    match unification {
                        None => { fact_indx += 1; },
                        Some(unifier) => {
                            let unifier = solve_unifier(&unifier);
                            // This clause matches!
                            nomatching = false;
                            // choose to take it
                            choice_points.push((
                                master.clone(), curr_query.clone(), fact_indx + 1
                            ));
                            // add the body of the clause to replace the
                            // front of our query, and union
                            // master and unifier
                            for (k, v) in unifier.iter() {
                                master.insert(k.clone(), v.clone());
                            }
                            let mut new_query_vec: Vec<Term> = clause.requires.goals.clone()
                                    .iter()
                                    .map(|goal| {
                                        let mut copy = goal.clone();
                                        copy.substitute_all(&unifier);
                                        copy
                                    })
                                    .collect();
                            new_query_vec.append(&mut curr_query.clone().goals[1..]
                                             .iter()
                                             .map(|other_goal| {
                                                let mut copy = other_goal.clone();
                                                copy.substitute_all(&unifier);
                                                copy
                                             }).collect());
                            new_query = Query { goals: new_query_vec };
                            fact_indx = 0;
                            break;
                        }
                    }
                }
                if nomatching {
                    match choice_points.pop() {
                        None => return None,
                        Some(choice_point) => {
                            master = choice_point.0;
                            curr_query = choice_point.1;
                            fact_indx = choice_point.2;
                            continue;
                        }
                    }
                }
            }
        }
        curr_query = new_query.clone();
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solver() {
        let fact1 = CompoundTerm {
            name: "member".to_string(),
            args: vec![
                Term::Unknown("X".to_string()),
                Term::List(ListTerm {
                    front: vec![
                        Term::Unknown("X".to_string())
                    ],
                    tail: ListTail::Unknown("Y".to_string())
                }),
            ]
        };
        let fact2 = CompoundTerm {
            name: "member".to_string(),
            args: vec![
                Term::Unknown("X2".to_string()),
                Term::List(ListTerm {
                    front: vec![
                        Term::Unknown("Z".to_string())
                    ],
                    tail: ListTail::Unknown("L2".to_string())
                }),
            ]
        };
        let requirement2 = vec![Term::Compound(CompoundTerm {
            name: "member".to_string(),
            args: vec![
                Term::Unknown("X2".to_string()),
                Term::Unknown("L2".to_string()),
            ]
        })];
        
        let facts = Rules {
            contents: vec![
                Rule {
                    gives: fact1,
                    requires: Query { goals: vec![] },
                },
                Rule {
                    gives: fact2,
                    requires: Query { goals: requirement2 },
                },
            ]
        };

        let query = Query {
            goals: vec![
                Term::Compound(CompoundTerm {
                    name: "member".to_string(),
                    args: vec![
                        Term::Unknown("FZ".to_string()),
                        Term::List(ListTerm {
                            front: vec![
                                Term::Atom("b".to_string()),
                                Term::Atom("c".to_string()),
                                Term::Atom("a".to_string()),
                            ],
                            tail: ListTail::End,
                        })
                    ]
                }),
                Term::Compound(CompoundTerm {
                    name: "member".to_string(),
                    args: vec![
                        Term::Unknown("FZ".to_string()),
                        Term::List(ListTerm {
                            front: vec![
                                Term::Atom("a".to_string()),
                                Term::Atom("d".to_string())
                            ],
                            tail: ListTail::End,
                        })
                    ]
                }),
            ]
        };
        println!("{:?}", solve(&facts, query));
    }
}
