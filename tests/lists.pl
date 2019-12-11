divide([], [], []).
divide([Var_x], [Var_x], []).
divide([Var_a, Var_b|Var_tail], Result0, Result1) :- divide(Var_tail, Var_ta, Var_tb), TmpdgQQaE = [Var_a|Var_ta],
	TmpGTTWKz = [Var_b|Var_tb],
	Result0 = TmpdgQQaE, Result1 = TmpGTTWKz.
merge([], Var_x, Var_x).
merge(Var_x, [], Result0) :- length(Var_x, TmpewxiBc),
	TmpewxiBc > 0, Result0 = Var_x.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a =< Var_b, TmpYgKgcS = [Var_b|Var_bs],
	merge(Var_as, TmpYgKgcS, TmpKEQURT),
	TmpxKhHeU = [Var_a|TmpKEQURT],
	Result0 = TmpxKhHeU.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a > Var_b, TmpfSwuOW = [Var_a|Var_as],
	merge(TmpfSwuOW, Var_bs, TmpFypNmi),
	TmplZaINF = [Var_b|TmpFypNmi],
	Result0 = TmplZaINF.
mergeSort([], []).
mergeSort([Var_x], [Var_x]).
mergeSort(Var_l, Result0) :- length(Var_l, TmpAshTYn),
	TmpAshTYn >= 2, divide(Var_l, Var_a, Var_b), mergeSort(Var_a, Tmpjjnjwz),
	mergeSort(Var_b, TmpoSpFdB),
	merge(Tmpjjnjwz, TmpoSpFdB, TmpuBWfQH),
	Result0 = TmpuBWfQH.
leaf(Var_x) :- atom(Var_x), TmpQpVXEj = [],
	Var_x \== TmpQpVXEj.

