pub mod ast;
pub mod span;
pub mod error;
pub mod checks;
pub mod solver;
pub mod prolog_print;

extern crate clap;
extern crate rand;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate annotate_snippets;
extern crate linefeed;
extern crate num_rational;
#[cfg(test)]
extern crate assert_cmd;
#[cfg(test)]
extern crate predicates;

use pest::Parser;

#[derive(Parser)]
#[grammar = "bevel.pest"]
pub struct BevelParser;

use ast::parse_program;
use crate::ast::parse::ParseNode;
use clap::{Arg, App};
use error::Error;
use std::fs;
use std::process::exit;
use std::io::{self, BufRead, Write};
use prolog_print::PrologPrint;
use linefeed::{Interface, ReadResult};

const REPL_FRAME_ID: u32 = 1;

#[cfg_attr(tarpaulin, skip)]
fn quit(e: Error) -> ! {
    eprintln!("{}", e);
    exit(1)
}

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("bevel")
            .version("0.1")
            .author("Matt Torrence <matt@torrencefamily.net>")
            .about("Bevel Programming Language")
            .arg(Arg::with_name("INPUT")
                 .help("The bevel source input")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("prolog_print")
                 .short("p")
                 .help("Print the equivelent prolog source code"))
            .arg(Arg::with_name("from_stdin")
                 .short("i")
                 .help("Read queries from stdin instead of initializing a REPL"))
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
    
    if matches.is_present("prolog_print") {
        let mut s = String::new();

        prog.prolog_print(&mut s).unwrap_or_else(|e| quit(e));

        println!("{}", s);
    } else {
        let mut prog_rules = solver::parse::parse_program(&prog);

        solver::optimize::apply_optimizations(&mut prog_rules);
        if matches.is_present("from_stdin") {
            let stdin = io::stdin();
            for input in stdin.lock().lines() {
                let input = input.unwrap();
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
                let mut state = solver::solve::new_solver_state(query);
                let solution = solver::solve::solve(&prog_rules, solver::solve::SolverState {
                    master: &mut state.master,
                    curr_query: &mut state.curr_query,
                    new_query: &mut state.new_query,
                    fact_indx: &mut state.fact_indx,
                    choice_points: &mut state.choice_points,
                });
                
                match &solution {
                    None => println!("fail"),
                    Some(solution) => {
                        let s = solver::fmt_unifier(&solution);
                        if s.trim().len() == 0 {
                            println!("success");
                        } else {
                            println!("{}", s);
                        }
                    },
                }
            }
        } else {
            println!();
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
                
                let mut state = solver::solve::new_solver_state(query);
                let mut line = String::new();
                let stdin = io::stdin();
                while !line.starts_with("q") {
                    let solution = solver::solve::solve(&prog_rules, solver::solve::SolverState {
                        master: &mut state.master,
                        curr_query: &mut state.curr_query,
                        new_query: &mut state.new_query,
                        fact_indx: &mut state.fact_indx,
                        choice_points: &mut state.choice_points,
                    });
                    
                    match &solution {
                        None => { println!("fail"); break; },
                        Some(solution) => {
                            let s = solver::fmt_unifier(&solution);
                            if s.trim().len() == 0 {
                                println!("success");
                                break;
                            } else {
                                print!("{} ", s);
                                io::stdout().flush().unwrap();

                                match state.choice_points.pop() {
                                    None => {
                                        println!("fail");
                                        break;
                                    },
                                    Some(choice_point) => {
                                        state.master = choice_point.0;
                                        state.curr_query = choice_point.1;
                                        state.fact_indx = choice_point.2;
                                    }
                                }
                            }
                        },
                    }
                    
                    stdin.lock().read_line(&mut line).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executions() {
        let fibsource =
r#"
fib(0) ~ 1;
fib(1) ~ 1;
fib(x) {
    x > 1
    relate fib(x - 1) + fib(x - 2)
};
"#.to_string();
        let fibquery = "x ~ fib(7), fib(7, y), (7, z) ~ fib()";
        let fibexpects = vec!["x = 21", "y = 21", "z = 21"];

        let heritagesource =
r#"
parent('matt) ~ 'kathy;
parent('kathy) ~ 'gdad;
parent('kathy) ~ 'gmom;
male() ~ 'matt;
male() ~ 'gdad;
female() ~ 'kathy;
female() ~ 'gmom;
grandfather(x) {
    gparent ~ parent(parent(x))
    male(gparent)
    relate gparent
};
"#.to_string();
        let hquery1 = "gfather ~ grandfather('kathy)";
        let hquery2 = "gfather ~ grandfather('matt)";
        let hquery3 = "nbody ~ grandfather('matt), nbody ~ parent('matt)";
        let hexpect1 = vec!["fail"];
        let hexpect2 = vec!["gfather = 'gdad"];
        let hexpect3 = vec!["fail"];

        let listsource =
r#"
head((x:_)) ~ x;

sameleading((x:y:z)) {
    x == y
};

samehead((x:_)) ~ (x:_);

swaptwo([x, y]) ~ [y, x];
"#.to_string();
        let listquery1 = "x ~ head([[1, 2], 3])";
        let listquery2 = "sameleading([1, 1, 200])";
        let listquery3 = "x ~ head([[1, 2], 3, 4]), y ~ head([[1, 3], 10, 5]), samehead(x, y)";
        let listexpect1 = vec!["x = [1, 2]"];
        let listexpect2 = vec!["success"];
        let listexpect3 = vec!["x = [1, 2]", "y = [1, 3]"];
        let sources = vec![&fibsource, &heritagesource, &heritagesource, &heritagesource, &listsource];
        let queries = vec![&fibquery, &hquery1, &hquery2, &hquery3, &listquery1, &listquery2, &listquery3];
        let expects = vec![&fibexpects, &hexpect1, &hexpect2, &hexpect3, &listexpect1, &listexpect2, &listexpect3];
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
            let mut state = solver::solve::new_solver_state(query);
            let solution = solver::solve::solve(&prog_rules, solver::solve::SolverState {
                master: &mut state.master,
                curr_query: &mut state.curr_query,
                new_query: &mut state.new_query,
                fact_indx: &mut state.fact_indx,
                choice_points: &mut state.choice_points,
            });
            match solution {
                Some(solution) => {
                    let asstr = solver::fmt_unifier(&solution);
                    if asstr.trim().len() == 0 {
                        assert!(expect[0].to_string() == "success".to_string());
                    } else {
                        for s in expect.iter() {
                            assert!(asstr.contains(s));
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
