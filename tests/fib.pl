fib(0, 1).
fib(1, 1).
fib(Var_x, Result0) :- Var_x > 1, TmpZbYyXk is Var_x - 1,
	fib(TmpZbYyXk, TmpfcwUrC),
	TmphlPKYU is Var_x - 2,
	fib(TmphlPKYU, TmpeOeDri),
	TmpgageEJ is TmpfcwUrC + TmpeOeDri,
	Result0 = TmpgageEJ.
myfun(0, 0).
myfun(0, 1).
myfun(Var_x, Result0) :- Var_x > 0, TmpxOEXuV is Var_x - 1,
	myfun(TmpxOEXuV, Var_y), TmpYzyBHm is Var_y + 1,
	Result0 = TmpYzyBHm.
justabove(Var_x, Result0) :- fib(5, Var_z), TmpXeNTtF is Var_x + Var_z,
	Result0 = TmpXeNTtF.
swapped(Var_x, Var_y, Var_y, Var_x).
parent(a, b).
parent(b, c).
parent(a, d).
grandparent(Var_x, Result0) :- parent(Var_x, TmphlAPmk),
	parent(TmphlAPmk, TmpNUCIIH),
	Result0 = TmpNUCIIH.
ancestor(Var_x, Var_x).
ancestor(Var_x, Result0) :- parent(Var_x, Var_y), ancestor(Var_y, TmpiFMKCx),
	Result0 = TmpiFMKCx.
branch(a, c, x, y).
branch(b, c, x, z).
calc(Var_x, Result0) :- fib(Var_x, TmpKEgJxJ),
	TmpvjhiZm is 10 * TmpKEgJxJ,
	TmpPFOzXY is TmpvjhiZm + 10,
	Var_y = TmpPFOzXY, Result0 = Var_y.
compute(Var_x, Var_x, 100).
main(Result0) :- compute(10, Var_x, Var_y), split(Var_x, Var_z, Var_w), write(Var_z), write(Var_w), TmpUFPqLO is Var_z + Var_w,
	TmpxFQScW is TmpUFPqLO + Var_y,
	Result0 = TmpxFQScW.
split(Var_x, Result0, Result1) :- TmpSZjvLs is Var_x + 1,
	TmpsoGERR is Var_x - 1,
	Result0 = TmpSZjvLs, Result1 = TmpsoGERR.

