fib(0, 1).
fib(1, 1).
fib(Var_x, Result0) :- Var_x > 1, TmpPqboZY is Var_x - 1,
	fib(TmpPqboZY, TmpRTTicF),
	TmpPDLRUw is Var_x - 2,
	fib(TmpPDLRUw, TmpIRaCBX),
	TmpkuvJyy is TmpRTTicF + TmpIRaCBX,
	Result0 = TmpkuvJyy.
myfun(0, 0).
myfun(0, 1).
myfun(Var_x, Result0) :- Var_x > 0, TmpzmVqAT is Var_x - 1,
	myfun(TmpzmVqAT, Var_y), TmpQbUBKy is Var_y + 1,
	Result0 = TmpQbUBKy.
justabove(Var_x, Result0) :- fib(5, Var_z), TmpNQwLDw is Var_x + Var_z,
	Result0 = TmpNQwLDw.
swapped(Var_x, Var_y, Var_y, Var_x).
parent(a, b).
parent(b, c).
parent(a, d).
grandparent(Var_x, Result0) :- parent(Var_x, TmpdPQobw),
	parent(TmpdPQobw, TmpEeEVhR),
	Result0 = TmpEeEVhR.
ancestor(Var_x, Var_x).
ancestor(Var_x, Result0) :- parent(Var_x, Var_y), ancestor(Var_y, TmpbRZktB),
	Result0 = TmpbRZktB.
branch(a, c, x, y).
branch(b, c, x, z).
calc(Var_x, Result0) :- fib(Var_x, TmpvVRLLB),
	TmpPXgBPg is 10 * TmpvVRLLB,
	TmpUloVxc is TmpPXgBPg + 10,
	Var_y = TmpUloVxc, Result0 = Var_y.
compute(Var_x, Var_x, 100).
main(Result0) :- compute(10, Var_x, Var_y), split(Var_x, Var_z, Var_w), write(Var_z), write(Var_w), TmpiDQTYv is Var_z + Var_w,
	TmpRUVctH is TmpiDQTYv + Var_y,
	Result0 = TmpRUVctH.
split(Var_x, Result0, Result1) :- TmpkBeWwn is Var_x + 1,
	TmpTopBZu is Var_x - 1,
	Result0 = TmpkBeWwn, Result1 = TmpTopBZu.
just([Var_x], Var_x).
head([Var_h, Var_t], Var_h).
flip([Var_a, Var_b], [Var_b, Var_a]).
newmain(Result0) :- TmpUKePrB = [1, 2],
	Var_x = TmpUKePrB, head(Var_x, TmpnqOPUf),
	TmpvkRIxh = [20],
	TmpMteAWl = [TmpvkRIxh, 30],
	TmpXxpuJt = [Var_x, TmpnqOPUf, 100, TmpMteAWl],
	Var_myotherlist = TmpXxpuJt, Result0 = Var_myotherlist.
reverse([], []).
reverse([Var_h|Var_tail], Result0) :- reverse(Var_tail, TmpojFNGM),
	TmpiEIALC = [Var_h],
	append(TmpojFNGM, TmpiEIALC, TmpckErQP),
	Result0 = TmpckErQP.
zip([], [], []).
zip([Var_h|Var_tail], [Var_h2|Var_tail2], Result0) :- zip(Var_tail, Var_tail2, TmptAEPrj),
	TmpZXAwHc = [Var_h, Var_h2|TmptAEPrj],
	Result0 = TmpZXAwHc.

