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
use crate::ast::parse::ParseNode;

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

extern crate linefeed;

use linefeed::{Interface, ReadResult};

const REPL_FRAME_ID: u32 = 1;

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
            .arg(Arg::with_name("repl")
                 .short("r")
                 .help("Start a repl loop"))
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
    
    if matches.is_present("repl") {
        let prog_rules = solver::parse::parse_program(&prog);

        // prog_rules.mangle_names();

        let reader = Interface::new("bevel").expect("Error setting up REPL loop. Something's gone very wrong!");

        reader.set_prompt("?#> ").expect("");

        while let ReadResult::Input(input) = reader.read_line().unwrap() {
            reader.add_history(input.clone());
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
            
            match &solution {
                None => println!("fail"),
                Some(solution) => println!("{}", solver::fmt_unifier(&solution)),
            }
        }
    } else {
        let mut s = String::new();

        prog.prolog_print(&mut s).unwrap_or_else(|e| quit(e));

        println!("{}", s);
    }
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
