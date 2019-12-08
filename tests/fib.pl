
fib(0, 1).
fib(1, 1).
fib(X, Y) :- X > 1,
	Xm1 is X - 1,
	Xm2 is X - 2,
	fib(Xm1, Y1),
	fib(Xm2, Y2),
	Y is Y1 + Y2.


swapped(X, Y, Y, X).

parent(a, b).
parent(b, c).
parent(a, d).

grandparent(X, Z) :- 
	parent(X, Y),
	parent(Y, Z).

ancestor(X, X).
ancestor(X, Z) :-
	parent(X, Y),
	ancestor(Y, Z).
