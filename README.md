# The Bevel Programming Language

Bevel is a logical declarative programming language inspired by [Prolog].

[Prolog]: https://en.wikipedia.org/wiki/Prolog

## Stability

Note that the programming language has still not been released. There are
no guarantees of language syntax and features being stable until the first 
release (0.1.0).

## Building and Usage

Install cargo and rust, then run `cargo build --release` from the root
directory. This will create a `bevel` executable, which can be run on a
bevel source file as follows:

```sh
$ bevel source.bv > source.pl
```

This will create a prolog source file to import. In the future, there will
be an executor that doesn't rely on prolog, but for now this is the only
way to run bevel code.

You can find examples of bevel code and syntax in the `tests` directory.
