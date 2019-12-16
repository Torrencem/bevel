
use crate::solver::*;
use crate::solver::solve::*;

use crate::solver::unify::compute_most_gen_unifier;

pub type Builtin = fn(&CompoundTerm, &SolverState) -> Option<Unifier>;

pub fn builtins() -> HashMap<String, Builtin> {
    let mut res = HashMap::new();
    res.insert("=".to_string(), 
               builtin_eq as Builtin);
    res.insert("+".to_string(),
               builtin_add as Builtin);
    res.insert("-".to_string(),
               builtin_sub as Builtin);
    res.insert("*".to_string(),
               builtin_mul as Builtin);
    res.insert("/".to_string(),
               builtin_div as Builtin);
    res.insert("%".to_string(),
               builtin_mod as Builtin);
    res.insert(">".to_string(),
               builtin_gt as Builtin);
    res.insert("<".to_string(),
               builtin_lt as Builtin);
    res.insert("<=".to_string(),
               builtin_leq as Builtin);
    res.insert(">=".to_string(),
               builtin_geq as Builtin);
    res.insert("==".to_string(),
               builtin_equ as Builtin);
    res.insert("!=".to_string(),
               builtin_neq as Builtin);
    res
}

pub fn builtin_eq(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    compute_most_gen_unifier(vec![(a, b)])
}

pub fn builtin_add(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 3);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    let c = cterm.args[2].clone();
    match (a, b, c) {
        (Term::Unknown(s),
        Term::Number(a),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b - a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Unknown(s),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b - a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Unknown(s)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a + b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Number(c)) => {
            if a + b == c {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_sub(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 3);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    let c = cterm.args[2].clone();
    match (a, b, c) {
        (Term::Unknown(s),
        Term::Number(a),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b + a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Unknown(s),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a - b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Unknown(s)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a - b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Number(c)) => {
            if a - b == c {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_mul(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 3);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    let c = cterm.args[2].clone();
    match (a, b, c) {
        (Term::Unknown(s),
        Term::Number(a),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b / a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Unknown(s),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b / a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Unknown(s)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a * b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Number(c)) => {
            if a * b == c {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_div(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 3);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    let c = cterm.args[2].clone();
    match (a, b, c) {
        (Term::Unknown(s),
        Term::Number(a),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(b * a));
            Some(unif)
        },
        (Term::Number(a),
        Term::Unknown(s),
        Term::Number(b)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a / b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Unknown(s)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a / b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Number(c)) => {
            if a / b == c {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_mod(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 3);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    let c = cterm.args[2].clone();
    match (a, b, c) {
        (Term::Number(a),
        Term::Number(b),
        Term::Unknown(s)) => {
            let mut unif = Unifier::new();
            unif.insert(s.clone(), Term::Number(a % b));
            Some(unif)
        },
        (Term::Number(a),
        Term::Number(b),
        Term::Number(c)) => {
            if a % b == c {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}


pub fn builtin_gt(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a > b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_lt(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a < b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_leq(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a <= b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_geq(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a >= b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_equ(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a == b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

pub fn builtin_neq(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    match (a, b) {
        (Term::Number(a),
        Term::Number(b)) => {
            if a != b {
                Some(Unifier::new())
            } else {
                None
            }
        },
        _ => {
            panic!("arguments to arithmetic not defined enough! \nthis will be a non-fatal error in the future")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_builtins_executions() {
        let formulasrc =
r#"
transform(z) {
    relate ((z + 2) * 3 / 4) % 5
};
aroundzero(x) {
    x < 1
    x > -1
    x <= 1
    x >= -1
    x != 1
    x == 0
};
"#.to_string();
        let formulaquery = "x ~ transform(10), aroundzero(0)";
        let formulaexpects = vec!["x = 4"];

        let sources = vec![&formulasrc];
        let queries = vec![&formulaquery];
        let expects = vec![&formulaexpects];
        for i in 0..sources.len() {
            let program_input = sources[i].clone();
            let input = queries[i].clone();
            let expect = expects[i].clone();
            let pairs = BevelParser::parse(Rule::program, &program_input).unwrap();
            
            let prog = parse_program(pairs, program_input.as_ref());
            
            let errs = checks::perform_checks(&prog, "test".to_string());
            let num_errs = errs.len();
            
            assert!(num_errs == 0);
            let mut prog_rules = solver::parse::parse_program(&prog);

            solver::optimize::apply_optimizations(&mut prog_rules);

            let raw_parse = BevelParser::parse(Rule::query, &input).expect("Error parsing input!"); // TODO
            let as_terms: Vec<solver::Term> = raw_parse.into_iter()
                .map(|pair| {
                    match pair.as_rule() {
                        Rule::assignment | Rule::mul_assignment => {
                            let rnode = ast::AssignmentNode::parse(pair, &input);
                            solver::parse::parse_assignment(&rnode, REPL_FRAME_ID).into_iter()
                        },
                        Rule::relation_call => {
                            let rcallnode = ast::RelationCallNode::parse(pair, &input);
                            solver::parse::parse_relationcall(&rcallnode, REPL_FRAME_ID).into_iter()
                        },
                        _ => unreachable!()
                    }
                })
                .flatten()
                .collect();
            let query = solver::Query {
                goals: as_terms
            };
            let solution = solver::solve::solve(&prog_rules, query);
            match solution {
                Some(solution) => {
                    let asstr = solver::fmt_unifier(&solution);
                    if asstr.trim().len() == 0 {
                        assert!(expect[0].to_string() == "success".to_string());
                    } else {
                        for s in expect.iter() {
                            assert!(asstr.contains(s), "{}", asstr);
                        }
                    }
                },
                None => {
                    assert!(expect[0] == "fail".to_string());
                }
            }
        }
    }
}
