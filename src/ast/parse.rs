
use crate::Rule;

use pest::iterators::{Pairs, Pair};

use pest::Span;

use pest::prec_climber::{PrecClimber, Operator, Assoc};

use crate::ast::*;

pub trait ParseNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self;
    fn as_span(&self) -> &Span<'p>;
}

pub fn parse_program<'p>(pairs: Pairs<'p, Rule>) -> ProgramNode<'p> {
    let relations: Vec<RelationNode<'p>> = 
        pairs.filter_map(|pair| {
            match pair.as_rule() {
                Rule::EOI => None,
                _ => Some(RelationNode::parse(pair))
            }
        }).collect();
    ProgramNode {
        relations: relations,
    }
}

impl<'p> ParseNode<'p> for RelationNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::relation_block => {
                let mut inners = pair.into_inner();
                let relation_decl = inners.next().unwrap();
                let mut relation_decl_breakdown = relation_decl.into_inner();
                let relation: RelationId<'p> =
                    RelationId::parse(relation_decl_breakdown.next().unwrap());
                let params: ConstList<'p> =
                    ConstList::parse(relation_decl_breakdown.next().unwrap());
                
                let block_or_const = inners.next().unwrap();
                let block: RelationBlock<'p> =
                    RelationBlock::parse(block_or_const);

                RelationNode {
                    span: span,
                    relation: relation,
                    params: params,
                    block: block
                }
            },
            Rule::simple_relation | Rule::multiple_relation => {
                let mut inners = pair.into_inner();
                let relation_decl = inners.next().unwrap();
                let mut relation_decl_breakdown = relation_decl.into_inner();
                let relation: RelationId<'p> =
                    RelationId::parse(relation_decl_breakdown.next().unwrap());
                let params: ConstList<'p> =
                    ConstList::parse(relation_decl_breakdown.next().unwrap());
                
                let block_or_const = inners.next().unwrap();
                let block: RelationBlock<'p> =
                    RelationBlock::parse(block_or_const);

                RelationNode {
                    span: span,
                    relation: relation,
                    params: params,
                    block: block
                }
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}


impl<'p> ParseNode<'p> for RelationBlock<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        match pair.as_rule() {
            Rule::block => {
                let block: BlockNode<'p> =
                    BlockNode::parse(pair);
                
                RelationBlock::Block(block)
            },
            Rule::pattern | Rule::pattern_list => {
                let const_node: ConstList<'p> =
                    ConstList::parse(pair);

                RelationBlock::Const(const_node)
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        match self {
            RelationBlock::Const(clist) => &clist.span,
            RelationBlock::Block(bnode) => &bnode.span,
        }
    }
}

impl<'p> ParseNode<'p> for RelationId<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::ident => {
                let name: String =
                    span.as_str().to_string();

                RelationId {
                    span: span,
                    name: name,
                }
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for ConstList<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::pattern_list => {
                let inner_const_terms = pair.into_inner();

                let constants: Vec<ConstantNode<'p>> =
                    inner_const_terms
                    .map(|pair| ConstantNode::parse(pair))
                    .collect();

                ConstList {
                    span: span,
                    constants: constants,
                }
            },
            Rule::pattern => {
                let mut inner_const_term = pair.into_inner();
                let constant: ConstantNode<'p> =
                    ConstantNode::parse(inner_const_term.next().unwrap());
                
                ConstList {
                    span: span,
                    constants: vec![constant],
                }
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for BlockNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::block => {
                let inner_statements = pair.into_inner();

                let statements: Vec<StatementNode<'p>> =
                    inner_statements
                    .map(|pair| StatementNode::parse(pair))
                    .collect();

                BlockNode {
                    span: span,
                    statements: statements,
                }
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for ConstantNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        
        let pair = 
            if Rule::pattern != pair.as_rule() {
                pair
            } else {
                pair.into_inner().next().unwrap()
            };
        
        ConstantNode {
            span: span.clone(),
            contents: {
                let ident: String =
                    span.as_str().to_string();
                match pair.as_rule() {
                    Rule::empty_pat => {
                        ConstantContents::EmptyPattern
                    },
                    Rule::atom => {
                        ConstantContents::Atom(ident)
                    },
                    Rule::ident => {
                        ConstantContents::Var(ident)
                    },
                    Rule::num_literal => {
                        ConstantContents::Literal(ident)
                    },
                    Rule::list_pattern => {
                        let innerds = pair.into_inner();
                        let contents: Vec<ConstantNode<'p>> =
                            innerds.map(|pair| ConstantNode::parse(pair))
                            .collect();
                        ConstantContents::List(contents)
                    },
                    Rule::conslist_pattern => {
                        let innerds = pair.into_inner();
                        let contents: Vec<ConstantNode<'p>> =
                            innerds.map(|pair| ConstantNode::parse(pair))
                            .collect();
                        ConstantContents::ConsList(contents)
                    },
                    x => panic!("unexpected: {:?} | {:?} | {:?}", x, pair, pair.as_span().lines().collect::<Vec<_>>()),
                }
            }
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for StatementNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        match pair.as_rule() {
            Rule::assignment | Rule::mul_assignment => {
                StatementNode::Assignment(AssignmentNode::parse(pair))
            },
            Rule::relate | Rule::mul_relate => {
                StatementNode::Relate(RelateNode::parse(pair))
            },
            Rule::binary_comparison => {
                StatementNode::BinaryFact(BinaryFactNode::parse(pair))
            },
            Rule::relation_call => {
                StatementNode::Relation(RelationCallNode::parse(pair))
            },
            x => panic!("unexpected: {:?}", x)
        }
    }

    fn as_span(&self) -> &Span<'p> {
        match self {
            StatementNode::Assignment(anode) => &anode.span,
            StatementNode::Relate(rnode) => &rnode.span,
            StatementNode::BinaryFact(bfnode) => &bfnode.span,
            StatementNode::Relation(rcallnode) => &rcallnode.span,
        }
    }
}

impl<'p> ParseNode<'p> for RelationCallNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span = pair.as_span();
        match pair.as_rule() {
            Rule::relation_call => {
                let mut innerds = pair.into_inner();
                let ident_pair = innerds.next().unwrap();
                let ident = RelationId::parse(ident_pair);
                
                let expr_list = innerds.next().unwrap();
                assert!(expr_list.as_rule() == Rule::expr_list);
                let innerds = expr_list.into_inner();

                let args: Vec<ExpressionNode<'p>> = innerds.map(|pair| {
                    ExpressionNode::parse(pair)
                }).collect();
                
                RelationCallNode {
                    rel: ident,
                    args: args,
                    span: span,
                }
            },
            x => panic!("unexpected: {:?}", x),
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for AssignmentNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::assignment => {
                let mut innerds = pair.into_inner();
                let constant_term = innerds.next().unwrap();
                let lhs: ConstantNode<'p> = ConstantNode::parse(constant_term);
                let expr_term = innerds.next().unwrap();
                let rhs: ExpressionNode<'p> = ExpressionNode::parse(expr_term);
                AssignmentNode {
                    span: span,
                    lhs: ConstList {
                        span: lhs.span.clone(),
                        constants: vec![ConstantNode {
                            span: lhs.span,
                            contents: lhs.contents,
                        }],
                    },
                    rhs: rhs,
                }
            },
            Rule::mul_assignment => {
                let mut innerds = pair.into_inner();
                let constant_term = innerds.next().unwrap();
                let lhs: ConstList<'p> = ConstList::parse(constant_term);
                let expr_term = innerds.next().unwrap();
                let rhs: ExpressionNode<'p> = ExpressionNode::parse(expr_term);
                AssignmentNode {
                    span: span,
                    lhs: lhs,
                    rhs: rhs,
                }
            },
            x => panic!("unexpected: {:?}", x)
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for RelateNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::relate => {
                let expr_term = pair.into_inner().next().unwrap();
                let result: ExpressionNode<'p> = 
                    ExpressionNode::parse(expr_term);
                RelateNode {
                    span: span,
                    result: vec![result]
                }
            },
            Rule::mul_relate => {
                let expr_term = pair.into_inner().next().unwrap();
                let exprs = expr_term.into_inner();
                let results: Vec<ExpressionNode<'p>> =
                    exprs
                    .map(|pair| { ExpressionNode::parse(pair) })
                    .collect();

                RelateNode {
                    span: span,
                    result: results
                }
            }
            x => panic!("unexpected: {:?}", x)
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for BinaryFactNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let span: Span<'p> = pair.as_span();
        match pair.as_rule() {
            Rule::binary_comparison => {
                let mut innerds = pair.into_inner();
                let left_expr = innerds.next().unwrap();
                let lhs: ExpressionNode<'p> =
                    ExpressionNode::parse(left_expr);
                let operator = innerds.next().unwrap();
                let op: BinaryFactOperation = match operator.as_rule() {
                    Rule::gt => BinaryFactOperation::Gt,
                    Rule::lt => BinaryFactOperation::Lt,
                    Rule::leq => BinaryFactOperation::Leq,
                    Rule::geq => BinaryFactOperation::Geq,
                    Rule::eq => BinaryFactOperation::Equ,
                    Rule::neq => BinaryFactOperation::Neq,
                    x => panic!("unexpected: {:?}", x),
                };
                let right_expr = innerds.next().unwrap();
                let rhs: ExpressionNode<'p> =
                    ExpressionNode::parse(right_expr);
                BinaryFactNode {
                    span: span,
                    lhs: lhs,
                    rhs: rhs,
                    op: op,
                }
            },
            x => panic!("unexpected: {:?}", x)
        }
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}

impl<'p> ParseNode<'p> for ExpressionNode<'p> {
    fn parse(pair: Pair<'p, Rule>) -> Self {
        let pairs = pair.into_inner();

        let climber = PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
            Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        ]);

        let primary = |pair: Pair<'p, Rule>| {
            ExpressionNode {
                span: pair.as_span(),
                contents: {
                    match pair.as_rule() {
                        Rule::num_literal | Rule::atom | Rule::ident =>
                            ExpressionContents::Const(ConstantNode::parse(pair)),
                        Rule::relation_call => {
                            let mut innerds = pair.into_inner();
                            let ident_pair = innerds.next().unwrap();
                            let ident = RelationId::parse(ident_pair);
                            
                            let expr_list = innerds.next().unwrap();
                            assert!(expr_list.as_rule() == Rule::expr_list);
                            let innerds = expr_list.into_inner();

                            let args: Vec<ExpressionNode<'p>> = innerds.map(|pair| {
                                ExpressionNode::parse(pair)
                            }).collect();
                            
                            ExpressionContents::Call {
                                rel: ident,
                                args: args,
                            }
                        },
                        Rule::list_expr => {
                            let innerds = pair.into_inner();
                            let vals: Vec<ExpressionNode<'p>> =
                                innerds.map(|pair| ExpressionNode::parse(pair))
                                .collect();
                            ExpressionContents::List {
                                vals: vals
                            }
                        },
                        Rule::conslist_expr => {
                            let innerds = pair.into_inner();
                            let vals: Vec<ExpressionNode<'p>> =
                                innerds.map(|pair| ExpressionNode::parse(pair))
                                .collect();
                            ExpressionContents::ConsList {
                                vals: vals
                            }
                        },
                        _ => {
                            // Parenthetical expression
                            let mut innerds = pair.into_inner();
                            let inner = innerds.next().unwrap();

                            ExpressionNode::parse(inner).contents
                        }
                    }
                },
            }
        };

        let infix = |lhs: ExpressionNode<'p>, op: Pair<'p, Rule>, rhs: ExpressionNode<'p>| {
            ExpressionNode {
                span: op.as_span(),
                contents: ExpressionContents::Operation {
                    op: match op.as_rule() {
                            Rule::add => BinaryOperation::Add,
                            Rule::subtract => BinaryOperation::Sub,
                            Rule::multiply => BinaryOperation::Mul,
                            Rule::divide => BinaryOperation::Div,
                            Rule::modulo => BinaryOperation::Mod,
                            x => panic!("unexpected {:?}", x),
                        },
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
        };
        
        climber.climb(pairs, primary, infix)
    }

    fn as_span(&self) -> &Span<'p> {
        &self.span
    }
}
