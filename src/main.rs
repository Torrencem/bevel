extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "bevel.pest"]
pub struct BevelParser;


mod ast;

use ast::parse_program;

fn main() {
    let program = r"fib(0) ~ 1;
fib(1) ~ 1;
fib(x) {
    x > 1
	relate fib(x - 1) + fib(x - 2)
};




% swapped(x, y) ~ (x, y);

parent('a) ~ 'b;
parent('b) ~ 'c;
parent('a) ~ 'd;

grandparent(x) {
	relate parent(parent(x))
};

ancestor(x) ~ x;
ancestor(x) {
	y ~ parent(x)
	relate ancestor(y)
};
    ";
    let pairs = BevelParser::parse(Rule::program, program).unwrap_or_else(|e| panic!("{}", e));
    
    let prog = parse_program(pairs);

    println!("{:#?}", prog);
}
