
pub mod parse;

pub use parse::parse_program;

use crate::span::Span;

pub trait ASTVisitor<State, Return> {
    fn visit_program(program: &ProgramNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for relation in program.relations.iter() {
            res.append(&mut Self::visit_relation(&relation, state));
        }
        res
    }

    fn visit_relation(relation: &RelationNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match &relation.block {
            RelationBlock::Const(clist) => {
                res.append(&mut Self::visit_constlist(&clist, state));
            },
            RelationBlock::Block(bnode) => {
                res.append(&mut Self::visit_block(&bnode, state));
            },
        };
        res
    }

    fn visit_relationid(_rid: &RelationId, _state: &State) -> Vec<Return> {
        vec![]
    }
    
    fn visit_constlist(clist: &ConstList, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for constant in clist.constants.iter() {
            res.append(&mut Self::visit_constant(&constant, state));
        }
        res
    }

    fn visit_block(bnode: &BlockNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for statement in bnode.statements.iter() {
            res.append(&mut Self::visit_statement(&statement, state));
        }
        res
    }

    fn visit_constant(constant: &ConstantNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        if let ConstantContents::List(listpat) = &constant.contents {
            for constant in listpat {
                res.append(&mut Self::visit_constant(&constant, state));
            }
        }
        if let ConstantContents::ConsList(listpat) = &constant.contents {
            for constant in listpat {
                res.append(&mut Self::visit_constant(&constant, state));
            }
        }
        res
    }

    fn visit_statement(statement: &StatementNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match statement {
            StatementNode::Assignment(anode) => {
                res.append(&mut Self::visit_assignment(&anode, state));
            },
            StatementNode::Relate(rnode) => {
                res.append(&mut Self::visit_relate(&rnode, state));
            },
            StatementNode::Refute(rnode) => {
                res.append(&mut Self::visit_refute(&rnode, state));
            },
            StatementNode::BinaryFact(bfact) => {
                res.append(&mut Self::visit_bfact(&bfact, state));
            },
            StatementNode::Relation(rcallnode) => {
                res.append(&mut Self::visit_relcall(&rcallnode, state));
            }
        }
        res
    }

    fn visit_assignment(assignment: &AssignmentNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut Self::visit_constlist(&assignment.lhs, state));
        res.append(&mut Self::visit_expr(&assignment.rhs, state));
        res
    }

    fn visit_relate(relate: &RelateNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for result in relate.result.iter() {
            res.append(&mut Self::visit_expr(&result, state));
        }
        res
    }

    fn visit_refute(refute: &RefuteNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut Self::visit_statement(&refute.statement, state));
        res
    }

    fn visit_bfact(bfact: &BinaryFactNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut Self::visit_expr(&bfact.lhs, state));
        res.append(&mut Self::visit_expr(&bfact.rhs, state));
        res
    }

    fn visit_expr(expression: &ExpressionNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match &expression.contents {
            ExpressionContents::Const(cnode) => {
                res.append(&mut Self::visit_constant(&cnode, state));
            },
            ExpressionContents::Operation { op: _op, lhs, rhs } => {
                res.append(&mut Self::visit_expr(&lhs, state));
                res.append(&mut Self::visit_expr(&rhs, state));
            },
            ExpressionContents::Call { rel, args } => {
                res.append(&mut Self::visit_relationid(&rel, state));
                for arg in args.iter() {
                    res.append(&mut Self::visit_expr(&arg, state));
                }
            },
            ExpressionContents::List { vals } => {
                for val in vals.iter() {
                    res.append(&mut Self::visit_expr(&val, state));
                }
            },
            ExpressionContents::ConsList { vals } => {
                for val in vals.iter() {
                    res.append(&mut Self::visit_expr(&val, state));
                }
            }
        }
        res
    }

    fn visit_relcall(rcall: &RelationCallNode, state: &State) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut Self::visit_relationid(&rcall.rel, state));
        for expr in rcall.args.iter() {
            res.append(&mut Self::visit_expr(&expr, state));
        }
        res
    }
}

#[derive(Debug)]
pub struct ProgramNode<'p> {
    pub relations: Vec<RelationNode<'p>>,
}

#[derive(Debug)]
pub struct RelationNode<'p> {
    pub span: Span<'p>,
    pub relation: RelationId<'p>,
    pub params: ConstList<'p>,
    pub block: RelationBlock<'p>,
}

#[derive(Debug)]
pub enum RelationBlock<'p> {
    Const(ConstList<'p>),
    Block(BlockNode<'p>),
}

#[derive(Debug)]
pub struct RelationId<'p> {
    pub span: Span<'p>,
    pub name: String,
}

#[derive(Debug)]
pub struct ConstList<'p> {
    pub span: Span<'p>,
    pub constants: Vec<ConstantNode<'p>>,
}

#[derive(Debug)]
pub struct BlockNode<'p> {
    pub span: Span<'p>,
    pub statements: Vec<StatementNode<'p>>,
}

pub fn find_num_results<'p>(bnode: &BlockNode<'p>) -> usize {
    for statement in bnode.statements.iter() {
        if let StatementNode::Relate(rnode) = statement {
            return rnode.result.len();
        }
    }
    return 0;
}

#[derive(Debug)]
pub struct ConstantNode<'p> {
    pub span: Span<'p>,
    pub contents: ConstantContents<'p>,
}

#[derive(Debug)]
pub enum ConstantContents<'p> {
    EmptyPattern,
    Atom(String),
    Var(String),
    Literal(String),
    List(Vec<ConstantNode<'p>>),
    ConsList(Vec<ConstantNode<'p>>),
}

#[derive(Debug)]
pub enum StatementNode<'p> {
    Assignment(AssignmentNode<'p>),
    Relate(RelateNode<'p>),
    Refute(RefuteNode<'p>),
    BinaryFact(BinaryFactNode<'p>),
    Relation(RelationCallNode<'p>),
}

#[derive(Debug)]
pub struct AssignmentNode<'p> {
    pub span: Span<'p>,
    pub lhs: ConstList<'p>,
    pub rhs: ExpressionNode<'p>,
}

#[derive(Debug)]
pub struct RelateNode<'p> {
    pub span: Span<'p>,
    pub result: Vec<ExpressionNode<'p>>,
}

#[derive(Debug)]
pub struct RefuteNode<'p> {
    pub span: Span<'p>,
    pub statement: Box<StatementNode<'p>>,
}

#[derive(Debug)]
pub struct BinaryFactNode<'p> {
    pub span: Span<'p>,
    pub lhs: ExpressionNode<'p>,
    pub rhs: ExpressionNode<'p>,
    pub op: BinaryFactOperation,
}

#[derive(Debug)]
pub enum BinaryFactOperation {
    Gt,
    Lt,
    Leq,
    Geq,
    Equ,
    Neq,
}

#[derive(Debug)]
pub struct ExpressionNode<'p> {
    pub span: Span<'p>,
    pub contents: ExpressionContents<'p>,
}

#[derive(Debug)]
pub enum ExpressionContents<'p> {
    Const(ConstantNode<'p>),
    Operation {
        op: BinaryOperation,
        lhs: Box<ExpressionNode<'p>>,
        rhs: Box<ExpressionNode<'p>>,
    },
    Call {
        rel: RelationId<'p>,
        args: Vec<ExpressionNode<'p>>,
    },
    List {
        vals: Vec<ExpressionNode<'p>>,
    },
    ConsList {
        vals: Vec<ExpressionNode<'p>>,
    },
}

#[derive(Debug)]
pub struct RelationCallNode<'p> {
    pub span: Span<'p>,
    pub rel: RelationId<'p>,
    pub args: Vec<ExpressionNode<'p>>,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
