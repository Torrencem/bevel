
use crate::Rule;

use pest::iterators::Pairs;

pub fn pprint(pairs: Pairs<Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::simple_relation => println!("SR"),
            Rule::relation_block => println!("RB"),
            Rule::EOI => {},
            _ => unreachable!(),
        }
    }
}

pub fn pprint_simple_relation(pair: Pair<Rule>) {
    
}
