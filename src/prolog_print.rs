
use crate::ast::*;

use crate::error::Result;

use std::fmt::Write;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::iter;

pub trait PrologPrint {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()>;
}

pub trait PrologPrintVal {
    fn prolog_print_val<W: Write>(&self, w: &mut W) -> Result<String>;
}

impl<'p> PrologPrint for ProgramNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        for relation in self.relations.iter() {
            relation.prolog_print(w)?;
            write!(w, ".\n")?;
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RelationNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
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
                let num_results = find_num_results(block);
                if pcomma && (num_results > 0) {
                    write!(w, ", ")?;
                }
                for i in 0..num_results {
                    if i == num_results - 1 {
                        write!(w, "Result{}", i)?;
                    } else {
                        write!(w, "Result{}, ", i)?;
                    }
                }
                write!(w, ") :- ")?;
                block.prolog_print(w)?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RelationId<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "{}", self.name)?;
        Ok(())
    }
}

impl<'p> PrologPrint for ConstList<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
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
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
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
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        match &self.contents {
            ConstantContents::EmptyPattern => {
                write!(w, "_")?;
            },
            ConstantContents::Atom(x) => {
                if x.chars().nth(1).unwrap().is_lowercase() {
                    write!(w, "{}", x[1..].to_string())?;
                } else {
                    write!(w, "atom_{}", x[1..].to_string())?;
                }
            },
            ConstantContents::Var(x) => write!(w, "Var_{}", x)?,
            ConstantContents::Literal(x) => write!(w, "{}", x)?,
            ConstantContents::List(l) => {
                write!(w, "[")?;
                let mut first = true;
                for c in l.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(w, ", ")?;
                    }
                    c.prolog_print(w)?;
                }
                write!(w, "]")?;
            },
            ConstantContents::ConsList(l) => {
                write!(w, "[")?;
                let mut first = true;
                for i in 0..(l.len() - 1) {
                    if first {
                        first = false;
                    } else {
                        write!(w, ", ")?;
                    }
                    l[i].prolog_print(w)?;
                }
                write!(w, "|")?;
                l[l.len() - 1].prolog_print(w)?;
                write!(w, "]")?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for StatementNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        match self {
            StatementNode::Assignment(anode) => anode.prolog_print(w)?,
            StatementNode::Relate(rnode) => rnode.prolog_print(w)?,
            StatementNode::Refute(rnode) => rnode.prolog_print(w)?,
            StatementNode::BinaryFact(bfnode) => bfnode.prolog_print(w)?,
            StatementNode::Relation(rnode) => rnode.prolog_print(w)?,
            StatementNode::TryOr(trnode) => trnode.prolog_print(w)?,
            StatementNode::Succeed => {
                write!(w, "true")?;
            },
            StatementNode::Fail => {
                write!(w, "false")?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RelationCallNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
            let mut arg_names = Vec::with_capacity(self.args.len());
            for arg in self.args.iter() {
                arg_names.push(arg.prolog_print_val(w)?);
            }
            self.rel.prolog_print(w)?;
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
            write!(w, ")")?;
            Ok(())
    }
}

impl<'p> PrologPrint for AssignmentNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
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
                write!(w, ")")?;
            },
            _ => {
                let list = &self.lhs.constants;
                if list.len() > 1 {
                    panic!("cannot assign to tuple: {}", self.span.as_str());
                }
                let result = self.rhs.prolog_print_val(w)?;
                list[0].prolog_print(w)?;
                write!(w, " = {}", result)?;
            }
        }
        Ok(())
    }
}

impl<'p> PrologPrint for RefuteNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "\\+ ")?;
        self.statement.prolog_print(w)?;
        Ok(())
    }
}
impl<'p> PrologPrint for RelateNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
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
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        let leftval = self.lhs.prolog_print_val(w)?;
        let rightval = self.rhs.prolog_print_val(w)?;
        let op = match self.op {
            BinaryFactOperation::Gt => ">",
            BinaryFactOperation::Lt => "<",
            BinaryFactOperation::Leq => "=<",
            BinaryFactOperation::Geq => ">=",
            BinaryFactOperation::Equ => "=:=",
            BinaryFactOperation::Neq => "\\==",
        };
        write!(w, "{} {} {}", leftval, op, rightval)?;
        Ok(())
    }
}

impl<'p> PrologPrint for TryOrNode<'p> {
    fn prolog_print<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "(")?;
        for i in 0..self.blocks.len() {
            self.blocks[i].prolog_print(w)?;
            if i != self.blocks.len() - 1 {
                write!(w, "\n  -> true\n;  ")?;
            }
        }
        write!(w, ")")?;
        Ok(())
    }
}

impl<'p> PrologPrintVal for ExpressionNode<'p> {
    fn prolog_print_val<W: Write>(&self, w: &mut W) -> Result<String> {
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
                    BinaryOperation::Mod => " mod ",
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
            ExpressionContents::List { vals } => {
                let mut names: Vec<String> =
                    Vec::with_capacity(vals.len());
                for val in vals {
                    names.push(val.prolog_print_val(w)?);
                }
                write!(w, "{} = [", name)?;
                for i in 0..vals.len() {
                    if i == vals.len() - 1 {
                        write!(w, "{}", names[i])?;
                    } else {
                        write!(w, "{}, ", names[i])?;
                    }
                }
                write!(w, "]")?;
            },
            ExpressionContents::ConsList { vals } => {
                let mut names: Vec<String> =
                    Vec::with_capacity(vals.len());
                for val in vals {
                    names.push(val.prolog_print_val(w)?);
                }
                write!(w, "{} = [", name)?;
                let mut first = true;
                for i in 0..(names.len() - 1) {
                    if first {
                        first = false;
                    } else {
                        write!(w, ", ")?;
                    }
                    write!(w, "{}", names[i])?;
                }
                write!(w, "|{}]", names[names.len() - 1])?;
            },
        }
        write!(w, ",\n\t")?;
        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::ast::parse_program;

    use assert_cmd::Command;
    use std;
    use std::fs::File;
    use std::io::{Write};
    use predicates::prelude::*;
    use predicate::str::contains;

    #[test]
    pub fn test_fib() -> Result<(), Box<dyn std::error::Error>> {
        let program_input =
r#"
fib(0) ~ 1;
fib(1) ~ 1;
fib(x) {
    x > 1
    relate fib(x - 1) + fib(x - 2)
};
"#.to_string();

        let pairs = BevelParser::parse(Rule::program, &program_input)?;

        let prog = parse_program(pairs, program_input.as_ref());

        let mut source = String::new();

        prog.prolog_print(&mut source)?;
        
        let mut file = File::create("__test__fib.pl")?;
        write!(file, "{}", source)?;

        Command::new("swipl")
            .arg("__test__fib.pl")
            .write_stdin("[__test__fib].\nfib(7, Y).")
            .assert()
            .stdout(contains("Y = 21"));
        
        std::fs::remove_file("__test__fib.pl")?;
        
        Ok(())
    }

    #[test]
    pub fn test_family() -> Result<(), Box<dyn std::error::Error>> {
        let program_input =
r#"
parent('matt) ~ 'kathy;
parent('kathy) ~ 'gdad;
parent('kathy) ~ 'gmom;
male() ~ 'matt;
male() ~ 'gdad;
female() ~ 'kathy;
female() ~ 'gmom;
grandfather(x) {
    gparent ~ parent(parent(x))
    male(gparent)
    relate gparent
};
"#.to_string();
        let pairs = BevelParser::parse(Rule::program, &program_input)?;

        let prog = parse_program(pairs, program_input.as_ref());

        let mut source = String::new();

        prog.prolog_print(&mut source)?;
        
        let mut file = File::create("__test__names.pl")?;
        write!(file, "{}", source)?;

        Command::new("swipl")
            .arg("__test__names.pl")
            .write_stdin("[__test__names].\ngrandfather(matt, Gfather).")
            .assert()
            .stdout(contains("Gfather = gdad"));
        
        Command::new("swipl")
            .arg("__test__names.pl")
            .write_stdin("[__test__names].\ngrandfather(kathy, Gfather).")
            .assert()
            .stdout(contains("false."));
        
        Command::new("swipl")
            .arg("__test__names.pl")
            .write_stdin("[__test__names].\ngrandfather(matt, Nbody), parent(matt, Nbody).")
            .assert()
            .stdout(contains("false."));
        
        std::fs::remove_file("__test__names.pl")?;
        
        Ok(())
    }

    #[test]
    pub fn test_listy() -> Result<(), Box<dyn std::error::Error>> {
        let program_input =
r#"
head((x:_)) ~ x;

sameleading((x:y:_)) {
    x == y
};

samehead((x:_)) ~ (x:_);

"#.to_string();
        let pairs = BevelParser::parse(Rule::program, &program_input)?;

        let prog = parse_program(pairs, program_input.as_ref());

        let mut source = String::new();

        prog.prolog_print(&mut source)?;
        
        let mut file = File::create("__test__listy.pl")?;
        write!(file, "{}", source)?;
        
        Command::new("swipl")
            .arg("__test__listy.pl")
            .write_stdin("[__test__listy].\nhead([[1, 2], 3], X).")
            .assert()
            .stdout(contains("X = [1, 2]"));
        
        Command::new("swipl")
            .arg("__test__listy.pl")
            .write_stdin("[__test__listy].\nsameleading([1, 1, 200]).")
            .assert()
            .stdout(contains("true."));
        
        Command::new("swipl")
            .arg("__test__listy.pl")
            .write_stdin("[__test__listy].\nsamehead([1, 2], [1, 3]).")
            .assert()
            .stdout(contains("true."));

        std::fs::remove_file("__test__listy.pl")?;

        Ok(())
    }
    
    #[test]
    pub fn test_misc() -> Result<(), Box<dyn std::error::Error>> {
        let program_input =
r#"
transform(z) {
    relate ((z + 2) * 3 / 4)
};
aroundzero(x) {
    x < 1
    x > -1
    x <= 1
    x >= -1
    x != 1
    x == 0
};
"#.to_string();
        let pairs = BevelParser::parse(Rule::program, &program_input)?;

        let prog = parse_program(pairs, program_input.as_ref());

        let mut source = String::new();

        prog.prolog_print(&mut source)?;
        
        let mut file = File::create("__test__misc.pl")?;
        write!(file, "{}", source)?;
        
        Command::new("swipl")
            .arg("__test__misc.pl")
            .write_stdin("[__test__misc].\ntransform(10, X).")
            .assert()
            .stdout(contains("X = 9"));
        
        Command::new("swipl")
            .arg("__test__misc.pl")
            .write_stdin("[__test__misc].\naroundzero(0).")
            .assert()
            .stdout(contains("true."));
        
        std::fs::remove_file("__test__misc.pl")?;

        Ok(())
    }
}
