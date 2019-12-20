
pub mod parse;

pub use parse::parse_program;

use crate::span::Span;

pub trait ASTVisitor<Return> {
    fn visit_program(&mut self, program: &ProgramNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for relation in program.relations.iter() {
            res.append(&mut self.visit_relation(&relation));
        }
        res
    }

    fn visit_relation(&mut self, relation: &RelationNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match &relation.block {
            RelationBlock::Const(clist) => {
                res.append(&mut self.visit_constlist(&clist));
            },
            RelationBlock::Block(bnode) => {
                res.append(&mut self.visit_block(&bnode));
            },
        };
        res
    }

    fn visit_relationid(&mut self, _rid: &RelationId) -> Vec<Return> {
        vec![]
    }
    
    fn visit_constlist(&mut self, clist: &ConstList) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for constant in clist.constants.iter() {
            res.append(&mut self.visit_constant(&constant));
        }
        res
    }

    fn visit_block(&mut self, bnode: &BlockNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for statement in bnode.statements.iter() {
            res.append(&mut self.visit_statement(&statement));
        }
        res
    }

    fn visit_constant(&mut self, constant: &ConstantNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        if let ConstantContents::List(listpat) = &constant.contents {
            for constant in listpat {
                res.append(&mut self.visit_constant(&constant));
            }
        }
        if let ConstantContents::ConsList(listpat) = &constant.contents {
            for constant in listpat {
                res.append(&mut self.visit_constant(&constant));
            }
        }
        res
    }

    fn visit_statement(&mut self, statement: &StatementNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match statement {
            StatementNode::Assignment(anode) => {
                res.append(&mut self.visit_assignment(&anode));
            },
            StatementNode::Relate(rnode) => {
                res.append(&mut self.visit_relate(&rnode));
            },
            StatementNode::Refute(rnode) => {
                res.append(&mut self.visit_refute(&rnode));
            },
            StatementNode::BinaryFact(bfact) => {
                res.append(&mut self.visit_bfact(&bfact));
            },
            StatementNode::Relation(rcallnode) => {
                res.append(&mut self.visit_relcall(&rcallnode));
            }
        }
        res
    }

    fn visit_assignment(&mut self, assignment: &AssignmentNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut self.visit_constlist(&assignment.lhs));
        res.append(&mut self.visit_expr(&assignment.rhs));
        res
    }

    fn visit_relate(&mut self, relate: &RelateNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        for result in relate.result.iter() {
            res.append(&mut self.visit_expr(&result));
        }
        res
    }

    fn visit_refute(&mut self, refute: &RefuteNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut self.visit_relcall(&*refute.statement));
        res
    }

    fn visit_bfact(&mut self, bfact: &BinaryFactNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut self.visit_expr(&bfact.lhs));
        res.append(&mut self.visit_expr(&bfact.rhs));
        res
    }

    fn visit_expr(&mut self, expression: &ExpressionNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        match &expression.contents {
            ExpressionContents::Const(cnode) => {
                res.append(&mut self.visit_constant(&cnode));
            },
            ExpressionContents::Operation { op: _op, lhs, rhs } => {
                res.append(&mut self.visit_expr(&lhs));
                res.append(&mut self.visit_expr(&rhs));
            },
            ExpressionContents::Call { rel, args } => {
                res.append(&mut self.visit_relationid(&rel));
                for arg in args.iter() {
                    res.append(&mut self.visit_expr(&arg));
                }
            },
            ExpressionContents::List { vals } => {
                for val in vals.iter() {
                    res.append(&mut self.visit_expr(&val));
                }
            },
            ExpressionContents::ConsList { vals } => {
                for val in vals.iter() {
                    res.append(&mut self.visit_expr(&val));
                }
            }
        }
        res
    }

    fn visit_relcall(&mut self, rcall: &RelationCallNode) -> Vec<Return> {
        let mut res: Vec<Return> = vec![];
        res.append(&mut self.visit_relationid(&rcall.rel));
        for expr in rcall.args.iter() {
            res.append(&mut self.visit_expr(&expr));
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
    pub statement: Box<RelationCallNode<'p>>,
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
