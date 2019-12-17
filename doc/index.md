# The Bevel Programming Language

Bevel is a declaritive programming language designed from the ground up to be as easy to read as possible. Heavily inspired by the Prolog language, Bevel shares most of it's functionality with Prolog and indeed can compile into it. However, as opposed to Prolog, the syntax of Bevel is a very intuitive and readable blend of functional and imperative syntax.

## Using Bevel

To build Bevel from source, make sure Rust and Cargo are installed, then download the source using git with `git clone https://github.com/Torrencem/bevel.git`. Then, navigate to the extracted source and build with `cargo build --release`. Use the compiled `bevel` binary with `bevel --help` to get usage information.

## A First Bevel Program: Fibonacci

For example, consider the following Bevel program:

```bevel
fib(0) ~ 0;
fib(1) ~ 1;
fib(n) {
	n > 1
	relate fib(n - 1) + fib(n - 2)
};
```

To run this example, save it to a file called `fib.bv`, and then run `bevel fib.bv` from the command line. You'll then see a read-eval-print prompt, which you can type queries such as `x ~ fib(7)` to compute relations.

## Fibonacci Explanation

Similarly to Prolog, a program is a list of facts, which describe relationships between terms. In this case, by saying `fib(0) ~ 0`, we're expressing that `0 ~ 0` where `~` is the `fib` relation. In Prolog, this would be expressed as `fib(0, 0).` Indeed, in Bevel, you can also write `fib(0, 0)`, which is equivelent to what was written, but for clarity it's encouraged to put "outputs" of relations at the end of lists of terms, and use the more specialized syntax as in our example. 

The final fact, which starts with `fib(n)`, expresses a requirement in order for the relationship `n ~ result` to hold. What follows in the block is a list of requirement relations that must be held. The final, `relate fib(n - 1) + fib(n - 2)`, is similar to `return` in imperative programming languages, since it requires the final argument (the "output") of the fib relationship to be equal to `fib(n - 1) + fib(n - 2)`.

Running the previous example in Prolog-print mode (`bevel -p file.bv`), we can see an equivelent example of Prolog code, which is essentially a du-sugared Bevel program:

```prolog
fib(0, 0).
fib(1, 1).
fib(Var_n, Result0) :- Var_n > 1, TmpnFPzrx is Var_n - 1,
        fib(TmpnFPzrx, TmpYIDZwV),
        TmpGlPCHH is Var_n - 2,
        fib(TmpGlPCHH, TmpplbFBB),
        TmpLywGfi is TmpYIDZwV + TmpplbFBB,
        Result0 = TmpLywGfi.
```

We can see that the two main pieces of very pleasent syntactical sugar Bevel provides are unpacking expressions from inside to outside in temporary variables and the `relate` keyword mentioned before. In fact, `relate` is actually much more powerful than `return` and gives a much more pleasent perspective to relations: unlike a `return` statement, relate can occur anywhere in a requirements block, or multiple times. Take the following example:

```bevel
element((x:_)) ~ x;
element((_:xs)) {
	relate element(xs)
};

common_element(la, lb) {
	relate element(la)
	relate element(lb)
};
```

In this example, for the `common_element` relation, we require that anything that satisfies our relation is both an element of `la` and an element of `lb`. Indeed, using our relations, whenever we want to require a term `x` to be a member of two lists, we can write either `common_element(la, lb, x)` or perhaps more readably `x ~ common_element(la, lb)` (these examples will work in the exact same way).

So, this example provides insight into how declarative programming is powerful: `element` acts somewhat like a function, but has "multiple outputs", and so is similar to a generator. Unlike a generator, no special syntax is needed to obtain all possible values from `element`, and the Bevel runtime will attempt to choose a value which will satisfy the contexts requirements as well.
