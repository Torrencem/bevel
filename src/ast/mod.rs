
pub mod parse;

pub use parse::parse_program;

use pest::Span;

#[derive(Debug)]
pub struct ProgramNode<'p> {
    relations: Vec<RelationNode<'p>>,
}

#[derive(Debug)]
pub struct RelationNode<'p> {
    span: Span<'p>,
    relation: RelationId<'p>,
    params: ConstList<'p>,
    block: RelationBlock<'p>,
}

#[derive(Debug)]
pub enum RelationBlock<'p> {
    Const(ConstantNode<'p>),
    Block(BlockNode<'p>),
}

#[derive(Debug)]
pub struct RelationId<'p> {
    span: Span<'p>,
    name: String,
}

#[derive(Debug)]
pub struct ConstList<'p> {
    span: Span<'p>,
    constants: Vec<ConstantNode<'p>>,
}

#[derive(Debug)]
pub struct BlockNode<'p> {
    span: Span<'p>,
    statements: Vec<StatementNode<'p>>,
}

#[derive(Debug)]
pub struct ConstantNode<'p> {
    span: Span<'p>,
    contents: ConstantContents,
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
}

#[derive(Debug)]
pub struct AssignmentNode<'p> {
    span: Span<'p>,
    lhs: ConstantNode<'p>,
    rhs: ExpressionNode<'p>,
}

#[derive(Debug)]
pub struct RelateNode<'p> {
    span: Span<'p>,
    result: ExpressionNode<'p>,
}

#[derive(Debug)]
pub struct BinaryFactNode<'p> {
    span: Span<'p>,
    lhs: ExpressionNode<'p>,
    rhs: ExpressionNode<'p>,
    op: BinaryFactOperation,
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
    span: Span<'p>,
    contents: ExpressionContents<'p>,
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
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div
}
