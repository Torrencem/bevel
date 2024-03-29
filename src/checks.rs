
use annotate_snippets::snippet::*;

use crate::ast::*;
use crate::error::{Error, Result, union_spans};
use crate::span::Span;

pub fn perform_checks(program: &ProgramNode, source: String) -> Vec<Error> {
    let mut result = vec![];
    result.append(&mut check_proc_relates(program, &source));
    result.append(&mut check_odd_ops(program, &source));
    result
}

// Traverse looking for odd operations (adding lists, etc.)
pub fn check_odd_ops(program: &ProgramNode, source: &String) -> Vec<Error> {
    (OddOps { state: source }).visit_program(program)
}

struct OddOps<'a> {state: &'a String}
impl<'a> ASTVisitor<Error> for OddOps<'a> {
    fn visit_expr(&mut self, expression: &ExpressionNode) -> Vec<Error> {
        let state = self.state;
        let mut res: Vec<Error> = vec![];
        match &expression.contents {
            ExpressionContents::Const(cnode) => {
                res.append(&mut self.visit_constant(&cnode));
            },
            ExpressionContents::Operation { op: _op, lhs, rhs } => {
                let mut invalid = false;
                if let ExpressionContents::List {..} = &lhs.contents {
                    invalid = true;
                }
                if let ExpressionContents::List {..} = &rhs.contents {
                    invalid = true;
                }
                if let ExpressionContents::ConsList {..} = &lhs.contents {
                    invalid = true;
                }
                if let ExpressionContents::ConsList {..} = &rhs.contents {
                    invalid = true;
                }

                if invalid {
                    let span = union_spans(&lhs.span, &rhs.span);
                    res.push(Error::Code(
                                check_odd_ops_snippet(&span, state)
                            ));
                }
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
}

// For each relation defined, check to make sure
// all the relates have the same 'return number'
fn check_proc_relates(program: &ProgramNode, source: &String) -> Vec<Error> {
    let mut result = vec![];
    for relation in program.relations.iter() {
        match &relation.block {
            RelationBlock::Const(..) => {},
            RelationBlock::Block(bnode) => {
                match check_relates_block(bnode, source) {
                    Err(e) => result.push(e),
                    Ok(()) => {}
                }
            },
        }
    }
    result
}

fn check_relates_block(block: &BlockNode, source: &String) -> Result<()> {
    let mut inferred_return_num: Option<usize> = None;
    let mut first_return: Option<Span> = None;
    for statement in block.statements.iter() {
        if let StatementNode::Relate(rnode) = &statement {
            match inferred_return_num {
                Some(val) => {
                    // Make sure the val is consistent
                    let this_val = rnode.result.len();
                    if val != this_val {
                        let top_span = first_return.unwrap();
                        let bottom_span = rnode.span.clone();
                        return Err(
                            Error::Code(
                                check_relate_error_snippet(&top_span, val, &bottom_span, this_val, source)
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

fn check_relate_error_snippet(span1: &Span, fst_retno: usize, span2: &Span, snd_retno: usize, source: &String) -> Snippet {
    let span = union_spans(span1, span2).from_line_begin();
    let starting_lno = span.line_no();
    let range1: (usize, usize) = (
            span.distance_from_start(span1.start), 
            span.distance_from_start(span1.end)
        );
    let range2: (usize, usize) = (
            span.distance_from_start(span2.start),
            span.distance_from_start(span2.end)
        );
    Snippet {
        title: Some(Annotation {
            label: Some("mismatched relates".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![
            Annotation {
                id: None,
                label: Some("all relates in a function must have the same number of outputs".to_string()),
                annotation_type: AnnotationType::Error,
            },
        ],
        slices: vec![
            Slice {
                source: span.as_str().to_string(),
                line_start: starting_lno,
                origin: Some(source.clone()),
                fold: true,
                annotations: vec![
                    SourceAnnotation {
                        range: range1,
                        label: format!("relates {} value{}", fst_retno, if fst_retno != 1 {"s"} else {""}).to_string(),
                        annotation_type: AnnotationType::Note
                    },
                    SourceAnnotation {
                        range: range2,
                        label: format!("relates {} value{}", snd_retno, if snd_retno != 1 {"s"} else {""}).to_string(),
                        annotation_type: AnnotationType::Note
                    },
                ],
            },
        ],
    }
}

fn check_odd_ops_snippet(span: &Span, source: &String) -> Snippet {
    let full_span = span.from_line_begin().to_line_end();
    let starting_lno = span.line_no();
    let range: (usize, usize) = (
            full_span.distance_from_start(span.start), 
            full_span.distance_from_start(span.end)
        );
    Snippet {
        title: Some(Annotation {
            label: Some("invalid operation".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![
            Annotation {
                id: None,
                label: Some("numeric operations can't be used on lists".to_string()),
                annotation_type: AnnotationType::Note,
            },
        ],
        slices: vec![
            Slice {
                source: full_span.as_str().to_string(),
                line_start: starting_lno,
                origin: Some(source.clone()),
                fold: false,
                annotations: vec![
                    SourceAnnotation {
                        range: range,
                        label: format!("invalid operation here").to_string(),
                        annotation_type: AnnotationType::Error
                    },
                ],
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    pub fn invoke_relate_mismatch() {
        let program_input =
r#"
impossible(x) {
    x > 1
    relate [1, x]
    relate (x, 2)
};
"#.to_string();
        let pairs = BevelParser::parse(Rule::program, &program_input).unwrap();

        let prog = parse_program(pairs, program_input.as_ref());

        let errs = checks::perform_checks(&prog, "test".to_string());
        assert!(errs.len() == 1);
        let err_msg = format!("{}", errs[0]).to_string();
        assert!(err_msg.contains("test"));
        assert!(err_msg.contains("relate (x, 2)"));
        assert!(err_msg.contains("relate [1, x]"));
    }

    #[test]
    pub fn invoke_oddops_error() {
        let program_input =
r#"
impossible(sthing) {
    relate sthing + [1, 2]
};
"#.to_string();
        let pairs = BevelParser::parse(Rule::program, &program_input).unwrap();

        let prog = parse_program(pairs, program_input.as_ref());

        let errs = checks::perform_checks(&prog, "test".to_string());
        assert!(errs.len() == 1);
        let err_msg = format!("{}", errs[0]).to_string();
        assert!(err_msg.contains("test"));
        assert!(err_msg.contains("sthing + [1, 2]"))
    }
}
