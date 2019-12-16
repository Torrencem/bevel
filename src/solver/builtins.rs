
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
