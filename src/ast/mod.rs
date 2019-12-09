
pub mod parse;

pub use parse::parse_program;

use pest::Span;

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
            // TODO: Maybe check consistency
            return rnode.result.len();
        }
    }
    panic!("block without relate statement! {}", bnode.span.as_str());
}

#[derive(Debug)]
pub struct ConstantNode<'p> {
    pub span: Span<'p>,
    pub contents: ConstantContents,
}

#[derive(Debug)]
pub enum ConstantContents {
    Atom(String),
    Var(String),
    Literal(String),
}

#[derive(Debug)]
pub enum StatementNode<'p> {
    Assignment(AssignmentNode<'p>),
    Relate(RelateNode<'p>),
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
}

#[derive(Debug)]
pub struct RelationCallNode<'p> {
    pub rel: RelationId<'p>,
    pub args: Vec<ExpressionNode<'p>>,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div
}
