
use crate::solver::*;
use crate::ast::*;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::iter;

// Utilities for turning ast trees into
// solvable structures

pub fn parse_program<'p>(pnode: &ProgramNode<'p>) -> Rules {
    let mut contents: Vec<Rule> = Vec::new();
    for rnode in pnode.relations.iter() {
        contents.push(parse_relation(&rnode));
    }
    Rules {
        contents: contents
    }
}

pub fn parse_relation<'p>(rnode: &RelationNode<'p>) -> Rule {
    let mut cterm: CompoundTerm = parse_relation_pattern(&rnode.relation, &rnode.params);
    let subquery: Query = parse_relation_block(&rnode.block, &mut cterm);
    Rule {
        gives: cterm,
        requires: subquery
    }
}

pub fn parse_relation_pattern<'p>(rid: &RelationId<'p>, clist: &ConstList<'p>) -> CompoundTerm {
    let my_terms: Vec<Term> = clist.constants.iter()
        .map(|cterm| {
            parse_constant(&cterm)
        })
        .collect();
    CompoundTerm {
        name: rid.name.clone(),
        args: my_terms
    }
}

pub fn parse_relation_block<'p>(rblock: &RelationBlock<'p>, cterm: &mut CompoundTerm) -> Query {
    match rblock {
        RelationBlock::Const(clist) => {
            let mut my_terms: Vec<Term> = clist.constants.iter()
                .map(|cterm| {
                    parse_constant(&cterm)
                })
                .collect();
            cterm.args.append(&mut my_terms);
            Query {
                goals: vec![]
            }
        },
        RelationBlock::Block(bnode) => {
            let num_results = find_num_results(&bnode);
            let mut new_args = (0..num_results).map(|num| {
                Term::Unknown(format!("Result{}", num))
            }).collect::<Vec<Term>>();
            cterm.args.append(&mut new_args);
            let mut goals: Vec<Term> = vec![];
            for statement in bnode.statements.iter() {
                let mut as_goals: Vec<Term> = parse_statement(&statement);
                goals.append(&mut as_goals);
            }
            Query {
                goals: goals
            }
        },
    }
}

pub fn parse_constant<'p>(cnode: &ConstantNode<'p>) -> Term {
    match &cnode.contents {
        ConstantContents::EmptyPattern => { unimplemented!() },
        ConstantContents::Atom(s) => Term::Atom(s.clone()),
        ConstantContents::Literal(s) => Term::Number(s.parse().unwrap()),
        ConstantContents::Var(s) => Term::Unknown(s.clone()),
        ConstantContents::List(vec) => {
            Term::List(ListTerm {
                front: vec.iter().map(|constant| {
                    parse_constant(constant)
                }).collect(),
                tail: ListTail::End,
            })
        },
        ConstantContents::ConsList(vec) => {
            Term::List(ListTerm {
                front: vec[..vec.len() - 1].iter().map(|constant| {
                    parse_constant(constant)
                }).collect(),
                tail: ListTail::Unknown({ 
                    if let Term::Unknown(s) = parse_constant(&vec[vec.len() - 1]) {
                        s.clone()
                    } else {
                        unreachable!()
                    }
                })
            })
        },
    }
}

pub fn parse_statement<'p>(statement: &StatementNode<'p>) -> Vec<Term> {
    match &statement {
        StatementNode::Assignment(anode) => parse_assignment(&anode),
        StatementNode::Relate(rnode) => parse_relate(&rnode),
        StatementNode::Refute(rnode) => unimplemented!(),
        StatementNode::BinaryFact(bfnode) => unimplemented!(),
        StatementNode::Relation(rcallnode) => parse_relationcall(&rcallnode),
    }
}

pub fn parse_assignment<'p>(assignment: &AssignmentNode<'p>) -> Vec<Term> {
    // If the rhs is not a compound, then make the assignment a special
    // compound =(X, Y) which (obviously) always resolves to X = Y
    match &assignment.rhs.contents {
        ExpressionContents::Call { rel, args } => {
            let mut res = vec![];
            let mut names = vec![];
            for expr in args {
                names.push(Term::Unknown(parse_expr_name(&expr, &mut res)));
            }
            let mut extra_args = assignment.lhs.constants.iter()
                .map(|constant| {
                    parse_constant(constant)
                }).collect();

            names.append(&mut extra_args);
            
            let assign_term = Term::Compound(CompoundTerm {
                name: rel.name.clone(),
                args: names,
            });
            res.push(assign_term);
            res
        },
        _ => {
            let mut res = vec![];
            let name = parse_expr_name(&assignment.rhs, &mut res);
            let assign_term = Term::Compound(CompoundTerm {
                name: "=".to_string(),
                args: vec![
                    parse_constant(&assignment.lhs.constants[0]),
                    Term::Unknown(name),
                ],
            });
            res.push(assign_term);
            res
        }
    }
}

pub fn parse_expr_name<'p>(expr: &ExpressionNode<'p>, res: &mut Vec<Term>) -> String {
    let mut rng = thread_rng();
    let name: String = format!("<Tmp>{}", iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .filter(|c| !c.is_digit(10))
        .take(6)
        .collect::<String>());
    match &expr.contents {
        ExpressionContents::Const(cnode) => {
            let cterm = parse_constant(&cnode);
            let assign_term = Term::Compound(CompoundTerm {
                name: "=".to_string(),
                args: vec![
                    cterm,
                    Term::Unknown(name.clone()),
                ],
            });
            res.push(assign_term);
        },
        ExpressionContents::Operation { op, lhs, rhs } => {
            let u1 = parse_expr_name(&lhs, res);
            let u2 = parse_expr_name(&rhs, res);
            let op_str = match op {
                BinaryOperation::Add => "+",
                BinaryOperation::Sub => "-",
                BinaryOperation::Mul => "*",
                BinaryOperation::Div => "/",
                BinaryOperation::Mod => "%",
            }.to_string();
            let op_term = Term::Compound(CompoundTerm {
                name: op_str,
                args: vec![
                    Term::Unknown(u1),
                    Term::Unknown(u2),
                    Term::Unknown(name.clone()),
                ],
            });
            res.push(op_term);
        },
        ExpressionContents::Call { rel, args } => {
            let mut names = vec![];
            for expr in args {
                names.push(Term::Unknown(parse_expr_name(&expr, res)));
            }
            names.push(Term::Unknown(name.clone()));
            let comp_term = Term::Compound(CompoundTerm {
                name: rel.name.clone(),
                args: names,
            });
            res.push(comp_term);
        },
        ExpressionContents::List { vals } => {
            let mut names = vec![];
            for expr in vals.iter() {
                names.push(Term::Unknown(parse_expr_name(&expr, res)));
            }
            let list_term = Term::List(ListTerm {
                front: names,
                tail: ListTail::End,
            });
            let assign_term = Term::Compound(CompoundTerm {
                name: "=".to_string(),
                args: vec![
                    list_term,
                    Term::Unknown(name.clone()),
                ]
            });
            res.push(assign_term);
        },
        ExpressionContents::ConsList { vals } => {
            let mut names = vec![];
            for expr in vals[..vals.len() - 1].iter() {
                names.push(Term::Unknown(parse_expr_name(&expr, res)));
            }
            let list_tail =
                match &vals[vals.len() - 1].contents {
                    ExpressionContents::List { vals } => {
                        for expr in vals {
                            names.push(Term::Unknown(parse_expr_name(&expr, res)));
                        }
                        ListTail::End
                    },
                    ExpressionContents::Const(cnode) => {
                        match &cnode.contents {
                            ConstantContents::Var(s) => {
                                ListTail::Unknown(s.clone())
                            },
                            ConstantContents::List(l) => {
                                for expr in l {
                                    let new_name: String = format!("Tmp{}", iter::repeat(())
                                        .map(|()| rng.sample(Alphanumeric))
                                        .filter(|c| !c.is_digit(10))
                                        .take(6)
                                        .collect::<String>());
                                    let cterm = parse_constant(expr);
                                    let assign_term = Term::Compound(CompoundTerm {
                                        name: "=".to_string(),
                                        args: vec![
                                            cterm,
                                            Term::Unknown(new_name.clone()),
                                        ],
                                    });
                                    res.push(assign_term);
                                }
                                ListTail::End
                            },
                            ConstantContents::ConsList(_) => {
                                unimplemented!()
                            },
                            _ => panic!()
                        }
                    },
                    _ => panic!()
                };
            let list_term = Term::List(ListTerm {
                front: names,
                tail: ListTail::End,
            });
            let assign_term = Term::Compound(CompoundTerm {
                name: "=".to_string(),
                args: vec![
                    list_term,
                    Term::Unknown(name.clone()),
                ]
            });
            res.push(assign_term);
        },
    }
    name
}

pub fn parse_relationcall<'p>(rcallnode: &RelationCallNode<'p>) -> Vec<Term> {
    let mut res = vec![];
    let mut names = vec![];
    for expr in rcallnode.args.iter() {
        names.push(Term::Unknown(parse_expr_name(&expr, &mut res)));
    }
    let cterm = Term::Compound(CompoundTerm {
        name: rcallnode.rel.name.clone(),
        args: names,
    });
    res.push(cterm);
    res
}

pub fn parse_relate<'p>(rnode: &RelateNode<'p>) -> Vec<Term> {
    let mut res = vec![];
    let mut names = vec![];
    for expr in rnode.result.iter() {
        names.push(Term::Unknown(parse_expr_name(&expr, &mut res)));
    }
    for i in 0..rnode.result.len() {
        let assign_term = Term::Compound(CompoundTerm {
            name: "=".to_string(),
            args: vec![
                Term::Unknown(format!("Result{}", i).to_string()),
                names[i].clone(),
            ]
        });
        res.push(assign_term);
    }
    res
}
