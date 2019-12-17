
extern crate assert_cmd;
extern crate predicates;
extern crate tempfile;

use assert_cmd::Command;
use std;
use std::io::{Write};
use tempfile::NamedTempFile;
use predicates::prelude::*;
use predicate::str::contains;

#[test]
pub fn test_fib() -> Result<(), Box<dyn std::error::Error>> {
    let source =
r#"
fib(0) ~ 1;
fib(1) ~ 1;
fib(x) {
	x > 1
	relate fib(x - 1) + fib(x - 2)
};
"#;
    
    let mut file = NamedTempFile::new()?;
    write!(file, "{}", source)?;

    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("y ~ fib(7)")
        .assert()
        .success()
        .stdout("y = 21\n");
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("y ~ fib(7), z ~ 2 * y - 20")
        .assert()
        .success()
        .stdout(contains("y = 21"))
        .stdout(contains("z = 22"));
    
    Ok(())
}

#[test]
pub fn test_family() -> Result<(), Box<dyn std::error::Error>> {
    let source =
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
"#;
    
    let mut file = NamedTempFile::new()?;
    write!(file, "{}", source)?;

    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("gfather ~ grandfather('matt)")
        .assert()
        .success()
        .stdout("gfather = 'gdad\n");
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("gfather ~ grandfather('kathy)")
        .assert()
        .success()
        .stdout("fail\n");
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("nbody ~ grandfather('matt), nbody ~ parent('matt)")
        .assert()
        .success()
        .stdout("fail\n");
    
    Ok(())
}

#[test]
pub fn test_listy() -> Result<(), Box<dyn std::error::Error>> {
    let source =
r#"
head((x:_)) ~ x;

sameleading((x:y:_)) {
    x == y
};

samehead((x:_)) ~ (x:_);

"#;
    
    let mut file = NamedTempFile::new()?;
    write!(file, "{}", source)?;

    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("x ~ head([[1, 2], 3])")
        .assert()
        .success()
        .stdout("x = [1, 2]\n");
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("sameleading([1, 1, 200])")
        .assert()
        .success()
        .stdout("success\n");
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("x ~ head([[1, 2], 3]), y ~ [1, 1+1*1], sameleading([x, y, 200])")
        .assert()
        .success()
        .stdout(contains("x = [1, 2]"))
        .stdout(contains("y = [1, 2]"));
    
    Command::cargo_bin("bevel")?
        .arg(file.path())
        .arg("-i")
        .write_stdin("x ~ head([[1, 2], 3, 4]), y ~ head([[1, 3], 10, 5]), samehead(x, y)")
        .assert()
        .success()
        .stdout(contains("x = [1, 2]"))
        .stdout(contains("y = [1, 3]"));

    Ok(())
}
