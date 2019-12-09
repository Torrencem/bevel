fib(0, 1).
fib(1, 1).
fib(Varx, Result) :- Varx > 1, TmpREYngJ is Varx - 1,
	fib(TmpREYngJ, TmpEQDSgZ),
	TmpAjPxqf is Varx - 2,
	fib(TmpAjPxqf, TmpjvYNjj),
	TmpOmWfjz is TmpEQDSgZ + TmpjvYNjj,
	Result = TmpOmWfjz.
myfun(0, 0).
myfun(0, 1).
myfun(Varx, Result) :- Varx > 0, TmpkdIWbQ is Varx - 1,
	myfun(TmpkdIWbQ, Vary), TmpKOrJTV is Vary + 1,
	Result = TmpKOrJTV.
justabove(Varx, Result) :- fib(5, Varz), TmplimdIc is Varx + Varz,
	Result = TmplimdIc.
swapped(Varx, Vary, Vary, Varx).
parent(atoma, atomb).
parent(atomb, atomc).
parent(atoma, atomd).
grandparent(Varx, Result) :- parent(Varx, TmpMiNGmF),
	parent(TmpMiNGmF, TmpbzNuLs),
	Result = TmpbzNuLs.
ancestor(Varx, Varx).
ancestor(Varx, Result) :- parent(Varx, Vary), ancestor(Vary, TmpFtTIio),
	Result = TmpFtTIio.
branch(atoma, atomc, atomx, atomy).
branch(atomb, atomc, atomx, atomz).
calc(Varx, Result) :- fib(Varx, TmpuUaUUy),
	TmppsmGcD is 10 * TmpuUaUUy,
	TmpglRRyD is TmppsmGcD + 10,
	Vary = TmpglRRyD, Result = Vary.

