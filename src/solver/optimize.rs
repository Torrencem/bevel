use crate::solver::*;

impl Rule {
    pub fn substitute_all(&mut self, un: &Unifier) -> Option<()> {
        for term in self.requires.goals.iter_mut() {
            term.substitute_all(un)?;
        }
        for term in self.gives.args.iter_mut() {
            term.substitute_all(un)?;
        }
        Some(())
    }
}

pub fn apply_optimizations(rules: &mut Rules) {
    for rule in rules.contents.iter_mut() {
        while apply_unneccessary_variable_opt(rule) {}
    }
}

pub fn apply_unneccessary_variable_opt(rule: &mut Rule) -> bool {
    let mut changed = false;
    let mut unif = Unifier::new();
    for term in rule.requires.goals.iter() {
        if let Term::Compound(CompoundTerm {
            name, args,
        }) = term {
            if *name == "=".to_string() {
                if let (Term::Unknown(a), Term::Unknown(b))
                    = (&args[0], &args[1]) {
                    if a.name.starts_with("_<") 
                        || a.name.starts_with("<Tmp>"){
                        unif.insert(b.clone(), Term::Unknown(a.clone()));
                    } else {
                        unif.insert(a.clone(), Term::Unknown(b.clone()));
                    }
                    changed = true;
                }
            }
        }
    }
    unif = solve_unifier(&unif);
    rule.substitute_all(&unif).unwrap();
    // Filter out x = x's
    let mut new_goals = vec![];
    for term in rule.requires.goals.iter() {
        if let Term::Compound(CompoundTerm {
            name, args,
        }) = term {
            if *name == "=".to_string() {
                if let (Term::Unknown(a), Term::Unknown(b))
                    = (&args[0], &args[1]) {
                    if a == b {
                        continue;
                    }
                }
            }
        }
        new_goals.push(term.clone());
    }
    rule.requires.goals = new_goals;
    changed
}
