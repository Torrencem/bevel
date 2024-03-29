
use crate::solver::*;
use crate::solver::unify::*;
use crate::solver::builtins::builtins;
use crate::REPL_FRAME_ID;
use rand::prelude::RngCore;
use rand::{thread_rng};

pub struct SolverStateOwner {
    pub master: Unifier,
    pub curr_query: Query,
    pub new_query: Query,
    pub fact_indx: usize,
    pub choice_points: Vec<(Unifier, Query, usize)>,
}

pub fn new_solver_state<'a>(query: Query) -> SolverStateOwner {
    SolverStateOwner {
        master: Unifier::new(),
        curr_query: query,
        new_query: Query { goals: vec![] },
        fact_indx: 0,
        choice_points: vec![]
    }
}

pub struct SolverState<'a> {
    pub master: &'a mut Unifier,
    pub curr_query: &'a mut Query,
    pub new_query: &'a mut Query,
    pub fact_indx: &'a mut usize,
    pub choice_points: &'a mut Vec<(Unifier, Query, usize)>,
}

pub fn solve(facts: &Rules, resume_state: SolverState) -> Option<Unifier> {
    let mut rng = thread_rng();
    let builtins = builtins();
    let fact_indx: &mut usize = resume_state.fact_indx;
    let master: &mut Unifier = resume_state.master;
    let curr_query: &mut Query = resume_state.curr_query;
    let new_query: &mut Query = resume_state.new_query;
    // A stack of querys, states, and fact indices
    let choice_points: &mut Vec<(Unifier, Query, usize)> = resume_state.choice_points;
    loop {
        match curr_query.goals.get(0) {
            None => {
                let unif = solve_unifier(&master);
                // Remove mangled names
                let mut filtered = Unifier::new();
                for (key, val) in unif.iter() {
                    if key.frame_id == REPL_FRAME_ID {
                        filtered.insert(key.clone(), val.clone());
                    }
                }
                return Some(filtered)
            },
            Some(goal) => {
                let mut skip = false;
                let mut nomatching = true;
                // Check for special goals here
                if let Term::Compound(cterm) = goal {
                    match builtins.get(&cterm.name) {
                        None => {},
                        Some(builtin) => {
                            let builtin_res =
                                builtin(&cterm);
                            match builtin_res {
                                None => {
                                    // backtrack
                                    match choice_points.pop() {
                                        None => return None,
                                        Some(choice_point) => {
                                            *master = choice_point.0;
                                            *curr_query = choice_point.1;
                                            *fact_indx = choice_point.2;
                                            continue;
                                        }
                                    }
                                },
                                Some(unifier) => {
                                    let unifier = solve_unifier(&unifier);
                                    for (k, v) in unifier.iter() {
                                        master.insert(k.clone(), v.clone());
                                    }
                                    let new_query_vec: Vec<Term> = curr_query.clone().goals[1..]
                                                     .iter()
                                                     .map(|other_goal| {
                                                        let mut copy = other_goal.clone();
                                                        copy.substitute_all(&unifier);
                                                        copy
                                                     }).collect();
                                    *new_query = Query { goals: new_query_vec };
                                    *fact_indx = 0;
                                    skip = true;
                                    nomatching = false;
                                },
                            }
                        }
                    }
                }
                if let Term::Refute(cterm) = goal {
                    skip = true;
                    // Try to solve "goal"
                    let query = Query {
                        goals: vec![Term::Compound(cterm.clone())],
                    };
                    let mut state = new_solver_state(query);
                    let solution = solve(facts, SolverState {
                        master: &mut state.master,
                        curr_query: &mut state.curr_query,
                        new_query: &mut state.new_query,
                        fact_indx: &mut state.fact_indx,
                        choice_points: &mut state.choice_points,
                    });
                    if let Some(_) = solution {
                        nomatching = true;
                    } else {
                        nomatching = false;
                    }
                    new_query.goals = curr_query.goals[1..].iter()
                        .cloned().collect();
                }
                // Find a clause that matches
                // the current goal
                if !skip {
                    for clause in facts.contents[*fact_indx..].iter() {
                        // println!("unify: {:?},\n{:?}", &goal, &clause.gives);
                        let unification =
                            compute_most_gen_unifier(vec![(goal.clone(), Term::Compound(clause.gives.clone()))]);
                        match unification {
                            None => { *fact_indx += 1; },
                            Some(unifier) => {
                                let mut unifier = solve_unifier(&unifier);
                                // This clause matches!
                                nomatching = false;
                                // choose to take it
                                choice_points.push((
                                    master.clone(), curr_query.clone(), *fact_indx + 1
                                ));
                                let new_frame_id = rng.next_u32();
                                for (_k, v) in unifier.iter_mut() {
                                    v.set_new_frame_id(new_frame_id);
                                }
                                // add the body of the clause to replace the
                                // front of our query, and union
                                // master and unifier
                                for (k, v) in unifier.iter() {
                                    master.insert(k.clone(), v.clone());
                                }
                                *master = solve_unifier(&master);
                                let mut new_query_vec: Vec<Term> = clause.requires.goals.clone()
                                        .iter()
                                        .map(|goal| {
                                            let mut copy = goal.clone();
                                            copy.substitute_all(&unifier);
                                            copy.set_new_frame_id(new_frame_id);
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
                                *new_query = Query { goals: new_query_vec };
                                *fact_indx = 0;
                                break;
                            }
                        }
                    }
                }
                if nomatching {
                    match choice_points.pop() {
                        None => return None,
                        Some(choice_point) => {
                            *master = choice_point.0;
                            *curr_query = choice_point.1;
                            *fact_indx = choice_point.2;
                            continue;
                        }
                    }
                }
            }
        }
        *curr_query = new_query.clone();
    }
}

