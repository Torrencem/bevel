
use crate::solver::*;
use crate::solver::solve::*;

use crate::solver::unify::compute_most_gen_unifier;

pub type Builtin = fn(&CompoundTerm, &SolverState) -> Option<Unifier>;

pub fn builtins() -> HashMap<String, Builtin> {
    let mut res = HashMap::new();
    res.insert("=".to_string(), 
               builtin_eq as Builtin);
    res
}

pub fn builtin_eq(cterm: &CompoundTerm, _state: &SolverState) -> Option<Unifier> {
    assert!(cterm.args.len() == 2);
    let a = cterm.args[0].clone();
    let b = cterm.args[1].clone();
    compute_most_gen_unifier(vec![(a, b)])
}

