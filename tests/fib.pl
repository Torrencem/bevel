fib(0, 1).
fib(1, 1).
fib(Var_x, Result0) :- Var_x > 1, TmpXtOKUV is Var_x - 1,
	fib(TmpXtOKUV, Tmpifpxmn),
	TmpPQOOxG is Var_x - 2,
	fib(TmpPQOOxG, TmpyaGblP),
	TmprTfKXq is Tmpifpxmn + TmpyaGblP,
	Result0 = TmprTfKXq.
myfun(0, 0).
myfun(0, 1).
myfun(Var_x, Result0) :- Var_x > 0, TmpyhpJBJ is Var_x - 1,
	myfun(TmpyhpJBJ, Var_y), TmpvaMbEP is Var_y + 1,
	Result0 = TmpvaMbEP.
justabove(Var_x, Result0) :- fib(5, Var_z), TmpewIyUI is Var_x + Var_z,
	Result0 = TmpewIyUI.
swapped(Var_x, Var_y, Var_y, Var_x).
parent(a, b).
parent(b, c).
parent(a, d).
grandparent(Var_x, Result0) :- parent(Var_x, TmpNZdmyc),
	parent(TmpNZdmyc, TmpBdUCBt),
	Result0 = TmpBdUCBt.
ancestor(Var_x, Var_x).
ancestor(Var_x, Result0) :- parent(Var_x, Var_y), ancestor(Var_y, TmpUPwoKf),
	Result0 = TmpUPwoKf.
branch(a, c, x, y).
branch(b, c, x, z).
calc(Var_x, Result0) :- fib(Var_x, TmpNqBVeH),
	TmpooNFXq is 10 * TmpNqBVeH,
	TmpSgDvhP is TmpooNFXq + 10,
	Var_y = TmpSgDvhP, Result0 = Var_y.
compute(Var_x, Var_x, 100).
main(Result0) :- compute(10, Var_x, Var_y), split(Var_x, Var_z, Var_w), TmpNdiIVL is Var_z + Var_w,
	TmpHneFid is TmpNdiIVL + Var_y,
	Result0 = TmpHneFid.
split(Var_x, Result0, Result1) :- TmpzJsZDB is Var_x + 1,
	TmpHSpVFN is Var_x - 1,
	Result0 = TmpzJsZDB, Result1 = TmpHSpVFN.

