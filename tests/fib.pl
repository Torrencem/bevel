fib(0, 1).
fib(1, 1).
fib(Var_x, Result0) :- Var_x > 1, TmpUUMneF is Var_x - 1,
	fib(TmpUUMneF, TmpGcMpVi),
	TmpISrJbl is Var_x - 2,
	fib(TmpISrJbl, TmpNTUCUE),
	TmpPbzUnZ is TmpGcMpVi + TmpNTUCUE,
	Result0 = TmpPbzUnZ.
myfun(0, 0).
myfun(0, 1).
myfun(Var_x, Result0) :- Var_x > 0, TmpXJqnrY is Var_x - 1,
	myfun(TmpXJqnrY, Var_y), TmpuUatec is Var_y + 1,
	Result0 = TmpuUatec.
justabove(Var_x, Result0) :- fib(5, Var_z), TmpVPtYej is Var_x + Var_z,
	Result0 = TmpVPtYej.
swapped(Var_x, Var_y, Var_y, Var_x).
parent(a, b).
parent(b, c).
parent(a, d).
grandparent(Var_x, Result0) :- parent(Var_x, TmpLvHUbI),
	parent(TmpLvHUbI, TmpFjvdAX),
	Result0 = TmpFjvdAX.
ancestor(Var_x, Var_x).
ancestor(Var_x, Result0) :- parent(Var_x, Var_y), ancestor(Var_y, TmpawnJPN),
	Result0 = TmpawnJPN.
branch(a, c, x, y).
branch(b, c, x, z).
calc(Var_x, Result0) :- fib(Var_x, TmpaybUkc),
	TmpOpMVZU is 10 * TmpaybUkc,
	TmpUChhwA is TmpOpMVZU + 10,
	Var_y = TmpUChhwA, Result0 = Var_y.
compute(Var_x, Var_x, 100).
main(Result0) :- compute(10, Var_x, Var_y), split(Var_x, Var_z, Var_w), write(Var_z), write(Var_w), TmpZoBfhk is Var_z + Var_w,
	TmpVioUpb is TmpZoBfhk + Var_y,
	Result0 = TmpVioUpb.
split(Var_x, Result0, Result1) :- TmpEovQZq is Var_x + 1,
	TmpDLAJuo is Var_x - 1,
	Result0 = TmpEovQZq, Result1 = TmpDLAJuo.
just([Var_x], Var_x).
head([Var_h, _], Var_h).
flip([Var_a, Var_b], [Var_b, Var_a]).
newmain(Result0) :- TmphRlbcY = [1, 2],
	Var_x = TmphRlbcY, head(Var_x, TmpvJErgd),
	TmpmDvZqj = [20],
	TmpcautmC = [TmpmDvZqj, 30],
	TmpWTsYnr = [Var_x, TmpvJErgd, 100, TmpcautmC],
	Var_myotherlist = TmpWTsYnr, Result0 = Var_myotherlist.
reverse([], []).
reverse([Var_h|Var_tail], Result0) :- reverse(Var_tail, TmpxkFZWw),
	TmpTUaSYk = [Var_h],
	append(TmpxkFZWw, TmpTUaSYk, TmpSPhSMP),
	Result0 = TmpSPhSMP.
zip([], [], []).
zip([Var_h|Var_tail], [Var_h2|Var_tail2], Result0) :- zip(Var_tail, Var_tail2, TmpARVNzf),
	TmpHUGBmf = [Var_h, Var_h2|TmpARVNzf],
	Result0 = TmpHUGBmf.

