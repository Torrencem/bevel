
extern crate clap;
use clap::{Arg, App};

extern crate rand;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "bevel.pest"]
pub struct BevelParser;

mod ast;
use ast::parse_program;

mod span;
mod error;
use error::Error;
mod checks;
mod solver;

use std::fs;
use std::process::exit;

mod prolog_print;
use prolog_print::PrologPrint;

extern crate annotate_snippets;

fn quit(e: Error) -> ! {
    eprintln!("{}", e);
    exit(1)
}

fn main() {
    let matches = App::new("bevel")
            .version("0.1")
            .author("Matt Torrence <matt@torrencefamily.net>")
            .about("Bevel Programming Language")
            .arg(Arg::with_name("INPUT")
                 .help("The bevel source input")
                 .required(true)
                 .index(1))
            .get_matches();
    
    let input_file = matches.value_of("INPUT").unwrap();

    // Replace tabs with spaces for formatting errors
    let program_input = fs::read_to_string(input_file).expect("error reading input file").replace("\t", "    ");

    let pairs = BevelParser::parse(Rule::program, &program_input).unwrap_or_else(|e| quit(e.into()));
    
    let prog = parse_program(pairs, program_input.as_ref());
    
    let errs = checks::perform_checks(&prog, input_file.to_string());
    let num_errs = errs.len();

    if num_errs > 0 {
        for err in errs {
            eprintln!("{}\n", err);
        }
        eprintln!("aborting due to the previous {} error{}", num_errs, if num_errs != 1 { "s" } else { "" });
        exit(1);
    }
    
    let mut s = String::new();

    prog.prolog_print(&mut s).unwrap_or_else(|e| quit(e));

    println!("{}", s);
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Rule;
    use crate::ast::parse_program;
    use crate::solver::parse;
    use crate::solver::solve::*;
    use crate::solver::*;

    #[test]
    fn test_parsing() {
        let program = r#"
parent('john) ~ 'jacob;
parent('mark) ~ 'john;

grandparent(x) {
    y ~ parent(x)
    z ~ parent(y)
    relate z
};
"#;
        let pairs = BevelParser::parse(Rule::program, program).unwrap();
        let prog = parse_program(pairs, program);
        let mut asrules = parse::parse_program(&prog);

        dbg!(&asrules);

        asrules.mangle_names();

        dbg!(&asrules);
        
        let query = Query {
            goals: vec![
                Term::Compound(CompoundTerm {
                    name: "grandparent".to_string(),
                    args: vec![
                        Term::Atom("'mark".to_string()),
                        Term::Unknown("who".to_string()),
                    ]
                })
            ]
        };

        let solution = solve(&asrules, query);
        dbg!(solution);
    }
}
