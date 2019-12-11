
use crate::ast::*;
use crate::ast::parse::ParseNode;
use crate::error::{Result, error_from_message_span};
use pest::Span;

pub fn perform_checks(program: &ProgramNode) -> Result<()> {
    check_proc_relates(program)
}

// For each relation defined, check to make sure
// all the relates have the same 'return number'
fn check_proc_relates(program: &ProgramNode) -> Result<()> {
    for relation in program.relations.iter() {
        match &relation.block {
            RelationBlock::Const(..) => {},
            RelationBlock::Block(bnode) => {
                check_relates_block(bnode)?;
            },
        }
    }
    Ok(())
}

fn check_relates_block(block: &BlockNode) -> Result<()> {
    let mut inferred_return_num: Option<usize> = None;
    let mut first_return: Option<Span> = None;
    for statement in block.statements.iter() {
        if let StatementNode::Relate(rnode) = &statement {
            match inferred_return_num {
                Some(val) => {
                    // Make sure the val is consistent
                    let this_val = rnode.result.len();
                    if val != this_val {
                        return Err(
                            error_from_message_span(
                                "relate inference error".to_string(),
                                format!("can't infer number of relates for relate body: {} != {}\n(earlier found '{}')", val, this_val, first_return.unwrap().as_str()),
                                rnode.span.clone()
                            )
                        );
                    }
                },
                None => {
                    inferred_return_num = Some(rnode.result.len());
                    first_return = Some(rnode.span.clone());
                },
            }
        }
    }
    Ok(())
}
