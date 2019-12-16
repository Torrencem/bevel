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
use std::io::{self, BufRead};

use prolog_print::PrologPrint;


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
                let solution = solver::solve::solve(&prog_rules, query);
                
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
        }
    }
}
