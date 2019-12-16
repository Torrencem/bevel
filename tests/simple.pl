inc(Var_a, Result0) :- TmpmjCAXb is Var_a + 1,
	Var_note = TmpmjCAXb, Result0 = Var_note.
main(Var_x, Result0) :- inc(Var_x, Var_y), inc(Var_y, Var_z), Result0 = Var_z.
ident(0, 0).
ident(Var_x, Result0) :- Var_x > 0, TmpJDVtNu is Var_x - 1,
	ident(TmpJDVtNu, TmpCsmBrZ),
	TmpmDAqQQ is TmpCsmBrZ + 1,
	Result0 = TmpmDAqQQ.
fib(0, 1).
fib(1, 1).
fib(Var_x, Result0) :- Var_x > 1, TmpizbnFk is Var_x - 1,
	fib(TmpizbnFk, TmpEqrOxG),
	TmpJSHQEz is Var_x - 2,
	fib(TmpJSHQEz, TmpgGikNB),
	TmpznwZZB is TmpEqrOxG + TmpgGikNB,
	Result0 = TmpznwZZB.

