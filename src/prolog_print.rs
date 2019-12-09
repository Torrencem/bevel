
use crate::ast::*;

use std::fmt;

use std::fmt::{Write, Error};

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::iter;

pub trait PrologPrint {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result;
}

pub trait PrologPrintVal {
    fn prolog_print_val<W: Write>(&self, w: &mut W) -> Result<String, Error>;
}

impl<'p> PrologPrint for ProgramNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        for relation in self.relations.iter() {
            relation.prolog_print(w)?;
            write!(w, ".\n")?;
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RelationNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        self.relation.prolog_print(w)?;
        write!(w, "(")?;
        self.params.prolog_print(w)?;
        let pcomma = self.params.constants.len() > 0;
        match &self.block {
            RelationBlock::Const(cnode) => {
                if pcomma {
                    write!(w, ", ")?;
                }
                cnode.prolog_print(w)?;
                write!(w, ")")?;
            },
            RelationBlock::Block(block) => {
                if pcomma {
                    write!(w, ", ")?;
                }
                let num_results = find_num_results(block);
                for i in 0..num_results {
                    if i == num_results - 1 {
                        write!(w, "Result{})", i)?;
                    } else {
                        write!(w, "Result{}, ", i)?;
                    }
                }
                write!(w, " :- ")?;
                block.prolog_print(w)?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RelationId<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{}", self.name)
    }
}

impl<'p> PrologPrint for ConstList<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        let mut first = true;

        for constant in self.constants.iter() {
            if !first {
                write!(w, ", ")?;
            } else {
                first = false;
            }
            constant.prolog_print(w)?;
        }
        Ok(())
    }
}

impl<'p> PrologPrint for BlockNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        let mut first = true;

        for statement in self.statements.iter() {
            if !first {
                write!(w, ", ")?;
            } else {
                first = false;
            }
            statement.prolog_print(w)?;
        }
        Ok(())
    }
}

impl<'p> PrologPrint for ConstantNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        match &self.contents {
            ConstantContents::Atom(x) => {
                if x.chars().nth(1).unwrap().is_lowercase() {
                    write!(w, "{}", x[1..].to_string())
                } else {
                    write!(w, "atom_{}", x[1..].to_string())
                }
            },
            ConstantContents::Var(x) => write!(w, "Var_{}", x),
            ConstantContents::Literal(x) => write!(w, "{}", x),
        }
    }
}

impl<'p> PrologPrint for StatementNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            StatementNode::Assignment(anode) => anode.prolog_print(w),
            StatementNode::Relate(rnode) => rnode.prolog_print(w),
            StatementNode::BinaryFact(bfnode) => bfnode.prolog_print(w),
        }
    }
}

impl<'p> PrologPrint for AssignmentNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        match &self.rhs.contents {
            ExpressionContents::Call {rel, args} => {
                let mut arg_names = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    arg_names.push(arg.prolog_print_val(w)?);
                }
                rel.prolog_print(w)?;
                write!(w, "(")?;
                let mut first = true;
                for arg in arg_names.iter() {
                    if !first {
                        write!(w, ", ")?;
                    } else {
                        first = false;
                    }
                    write!(w, "{}", arg)?;
                }
                if !first {
                    write!(w, ", ")?;
                }
                self.lhs.prolog_print(w)?;
                write!(w, ")")
            },
            _ => {
                let list = &self.lhs.constants;
                if list.len() > 1 {
                    panic!("cannot assign to tuple: {}", self.span.as_str());
                }
                let result = self.rhs.prolog_print_val(w)?;
                list[0].prolog_print(w)?;
                write!(w, " = {}", result)
            }
        }
    }
}

impl<'p> PrologPrint for RelateNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        let mut res: Vec<String> = Vec::with_capacity(self.result.len());
        for val in self.result.iter() {
            res.push(val.prolog_print_val(w)?);
        }
        for i in 0..self.result.len() {
            if i == self.result.len() - 1 {
                write!(w, "Result{} = {}", i, res[i])?;
            } else {
                write!(w, "Result{} = {}, ", i, res[i])?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for BinaryFactNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> fmt::Result {
        let leftval = self.lhs.prolog_print_val(w)?;
        let rightval = self.rhs.prolog_print_val(w)?;
        let op = match self.op {
            BinaryFactOperation::Gt => ">",
            BinaryFactOperation::Lt => "<",
            BinaryFactOperation::Leq => "=<",
            BinaryFactOperation::Geq => ">=",
            BinaryFactOperation::Equ => "=",
        };
        write!(w, "{} {} {}", leftval, op, rightval)
    }
}

impl<'p> PrologPrintVal for ExpressionNode<'p> {
    fn prolog_print_val<W: Write>(&self, w: &mut W) -> Result<String, Error> {
        if let ExpressionContents::Const(cnode) = &self.contents {
            let mut s = String::new();
            cnode.prolog_print(&mut s)?;
            return Ok(s);
        }
        let mut rng = thread_rng();
        let name: String = format!("Tmp{}", iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .filter(|c| !c.is_digit(10))
            .take(6)
            .collect::<String>());
        match &self.contents {
            ExpressionContents::Const(_) => unreachable!(),
            ExpressionContents::Operation { op, lhs, rhs } => {
                let oper = match op {
                    BinaryOperation::Add => "+",
                    BinaryOperation::Sub => "-",
                    BinaryOperation::Mul => "*",
                    BinaryOperation::Div => "/",
                };
                let left = lhs.prolog_print_val(w)?;
                let right = rhs.prolog_print_val(w)?;
                write!(w, "{} is {} {} {}", name, left, oper, right)?;
            },
            ExpressionContents::Call { rel, args } => {
                let mut arg_names = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    arg_names.push(arg.prolog_print_val(w)?);
                }
                rel.prolog_print(w)?;
                write!(w, "(")?;
                let mut first = true;
                for arg in arg_names.iter() {
                    if !first {
                        write!(w, ", ")?;
                    } else {
                        first = false;
                    }
                    write!(w, "{}", arg)?;
                }
                if !first {
                    write!(w, ", ")?;
                }
                write!(w, "{}", name)?;
                write!(w, ")")?;
            },
        }
        write!(w, ",\n\t")?;
        Ok(name)
    }
}
