
use crate::solver::*;
use crate::ast::*;

use rand::prelude::RngCore;
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
    let mut rng = thread_rng();
    let frame_id: u32 = rng.next_u32();
    let mut cterm: CompoundTerm = parse_relation_pattern(&rnode.relation, &rnode.params, frame_id);
    let subquery: Query = parse_relation_block(&rnode.block, frame_id, &mut cterm);
    Rule {
        gives: cterm,
        requires: subquery
    }
}

pub fn parse_relation_pattern<'p>(rid: &RelationId<'p>, clist: &ConstList<'p>, frame_id: u32) -> CompoundTerm {
    let my_terms: Vec<Term> = clist.constants.iter()
        .map(|cterm| {
            parse_constant(&cterm, frame_id)
        })
        .collect();
    CompoundTerm {
        name: rid.name.clone(),
        args: my_terms
    }
}

pub fn parse_relation_block<'p>(rblock: &RelationBlock<'p>, frame_id: u32, cterm: &mut CompoundTerm) -> Query {
    match rblock {
        RelationBlock::Const(clist) => {
            let mut my_terms: Vec<Term> = clist.constants.iter()
                .map(|cterm| {
                    parse_constant(&cterm, frame_id)
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
                Term::Unknown(UnknownContents {
                    name: format!("Result{}", num),
                    frame_id: frame_id
                })
            }).collect::<Vec<Term>>();
            cterm.args.append(&mut new_args);
            let mut goals: Vec<Term> = vec![];
            for statement in bnode.statements.iter() {
                let mut as_goals: Vec<Term> = parse_statement(&statement, frame_id);
                goals.append(&mut as_goals);
            }
            Query {
                goals: goals
            }
        },
    }
}

pub fn parse_constant<'p>(cnode: &ConstantNode<'p>, frame_id: u32) -> Term {
    match &cnode.contents {
        ConstantContents::EmptyPattern => {
            let mut rng = thread_rng();
            let name = UnknownContents {
                name: format!("<Tmp_WC>{}", iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .filter(|c| !c.is_digit(10))
                    .take(6)
                    .collect::<String>()),
                frame_id: frame_id,
            };
            Term::Unknown(name)
        },
        ConstantContents::Atom(s) => Term::Atom(s.clone()),
        ConstantContents::Literal(s) => Term::Number(s.parse().unwrap()),
        ConstantContents::Var(s) => Term::Unknown(UnknownContents {
            name: s.clone(),
            frame_id: frame_id
        }),
        ConstantContents::List(vec) => {
            Term::List(ListTerm {
                front: vec.iter().map(|constant| {
                    parse_constant(constant, frame_id)
                }).collect(),
                tail: ListTail::End,
            })
        },
        ConstantContents::ConsList(vec) => {
            Term::List(ListTerm {
                front: vec[..vec.len() - 1].iter().map(|constant| {
                    parse_constant(constant, frame_id)
                }).collect(),
                tail: ListTail::Unknown({
                    if let Term::Unknown(s) = parse_constant(&vec[vec.len() - 1], frame_id) {
                        s.clone()
                    } else {
                        unreachable!()
                    }
                })
            })
        },
    }
}

pub fn parse_statement<'p>(statement: &StatementNode<'p>, frame_id: u32) -> Vec<Term> {
    match &statement {
        StatementNode::Assignment(anode) => parse_assignment(&anode, frame_id),
        StatementNode::Relate(rnode) => parse_relate(&rnode, frame_id),
        StatementNode::Refute(_) => unimplemented!(),
        StatementNode::BinaryFact(brnode) => parse_bfactnode(&brnode, frame_id),
        StatementNode::Relation(rcallnode) => parse_relationcall(&rcallnode, frame_id),
    }
}

pub fn parse_bfactnode<'p>(brnode: &BinaryFactNode, frame_id: u32) -> Vec<Term> {
    let mut res = vec![];
    let left_name = parse_expr_name(&brnode.lhs, frame_id, &mut res);
    let right_name = parse_expr_name(&brnode.rhs, frame_id, &mut res);
    let op_name = match brnode.op {
        BinaryFactOperation::Gt => ">".to_string(),
        BinaryFactOperation::Lt => "<".to_string(),
        BinaryFactOperation::Leq => "<=".to_string(),
        BinaryFactOperation::Geq => ">=".to_string(),
        BinaryFactOperation::Equ => "==".to_string(),
        BinaryFactOperation::Neq => "!=".to_string(),
    };
    let comp_term = Term::Compound(CompoundTerm {
        name: op_name,
        args: vec![
            Term::Unknown(left_name),
            Term::Unknown(right_name),
        ]
    });
    res.push(comp_term);
    res
}

pub fn parse_assignment<'p>(assignment: &AssignmentNode<'p>, frame_id: u32) -> Vec<Term> {
    // If the rhs is not a compound, then make the assignment a special
    // compound =(X, Y) which (obviously) always resolves to X = Y
    match &assignment.rhs.contents {
        ExpressionContents::Call { rel, args } => {
            let mut res = vec![];
            let mut names = vec![];
            for expr in args {
                names.push(Term::Unknown(parse_expr_name(&expr, frame_id, &mut res)));
            }
            let mut extra_args = assignment.lhs.constants.iter()
                .map(|constant| {
                    parse_constant(constant, frame_id)
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
            let name = parse_expr_name(&assignment.rhs, frame_id, &mut res);
            let assign_term = Term::Compound(CompoundTerm {
                name: "=".to_string(),
                args: vec![
                    parse_constant(&assignment.lhs.constants[0], frame_id),
                    Term::Unknown(name),
                ],
            });
            res.push(assign_term);
            res
        }
    }
}

pub fn parse_expr_name<'p>(expr: &ExpressionNode<'p>, frame_id: u32, res: &mut Vec<Term>) -> UnknownContents {
    let mut rng = thread_rng();
    let name = UnknownContents {
        name: format!("<Free>{}", iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .filter(|c| !c.is_digit(10))
            .take(6)
            .collect::<String>()),
        frame_id: frame_id,
    };
    match &expr.contents {
        ExpressionContents::Const(cnode) => {
            let cterm = parse_constant(&cnode, frame_id);
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
            let u1 = parse_expr_name(&lhs, frame_id, res);
            let u2 = parse_expr_name(&rhs, frame_id, res);
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
                names.push(Term::Unknown(parse_expr_name(&expr, frame_id, res)));
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
                names.push(Term::Unknown(parse_expr_name(&expr, frame_id, res)));
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
                names.push(Term::Unknown(parse_expr_name(&expr, frame_id, res)));
            }
            let list_tail =
                match &vals[vals.len() - 1].contents {
                    ExpressionContents::List { vals } => {
                        for expr in vals {
                            names.push(Term::Unknown(parse_expr_name(&expr, frame_id, res)));
                        }
                        ListTail::End
                    },
                    ExpressionContents::Const(cnode) => {
                        match &cnode.contents {
                            ConstantContents::Var(s) => {
                                ListTail::Unknown(UnknownContents {
                                    name: s.clone(),
                                    frame_id: frame_id
                                })
                            },
                            ConstantContents::List(l) => {
                                for expr in l {
                                    let new_name = UnknownContents {
                                        name: format!("Tmp{}", iter::repeat(())
                                            .map(|()| rng.sample(Alphanumeric))
                                            .filter(|c| !c.is_digit(10))
                                            .take(6)
                                            .collect::<String>()),
                                        frame_id: frame_id
                                    };
                                    let cterm = parse_constant(expr, frame_id);
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
                tail: list_tail,
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

pub fn parse_relationcall<'p>(rcallnode: &RelationCallNode<'p>, frame_id: u32) -> Vec<Term> {
    let mut res = vec![];
    let mut names = vec![];
    for expr in rcallnode.args.iter() {
        names.push(Term::Unknown(parse_expr_name(&expr, frame_id, &mut res)));
    }
    let cterm = Term::Compound(CompoundTerm {
        name: rcallnode.rel.name.clone(),
        args: names,
    });
    res.push(cterm);
    res
}

pub fn parse_relate<'p>(rnode: &RelateNode<'p>, frame_id: u32) -> Vec<Term> {
    let mut res = vec![];
    let mut names = vec![];
    for expr in rnode.result.iter() {
        names.push(Term::Unknown(parse_expr_name(&expr, frame_id, &mut res)));
    }
    for i in 0..rnode.result.len() {
        let assign_term = Term::Compound(CompoundTerm {
            name: "=".to_string(),
            args: vec![
                Term::Unknown(UnknownContents {
                    name: format!("Result{}", i).to_string(),
                    frame_id: frame_id
                }),
                names[i].clone(),
            ]
        });
        res.push(assign_term);
    }
    res
}
