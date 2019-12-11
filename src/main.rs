
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
mod checks;

use std::fs;
use std::process::exit;

mod prolog_print;
use prolog_print::PrologPrint;

extern crate annotate_snippets;

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
    let program_input = fs::read_to_string(input_file).expect("Error reading input file").replace("\t", "    ");

    let pairs = BevelParser::parse(Rule::program, &program_input).unwrap_or_else(|e| panic!("{}", e));
    
    let prog = parse_program(pairs, program_input.as_ref());
    
    checks::perform_checks(&prog, input_file.to_string()).unwrap_or_else(|e| {
        // TODO
        eprintln!("{}", e);
        exit(1);
    });
    
    let mut s = String::new();

    prog.prolog_print(&mut s).expect("IO error when writing to buffer");

    println!("{}", s);
}
