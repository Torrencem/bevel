divide([], [], []).
divide([Var_x], [Var_x], []).
divide([Var_a, Var_b|Var_tail], Result0, Result1) :- divide(Var_tail, Var_ta, Var_tb), TmpDmntrW = [Var_a|Var_ta],
	TmpGwiWWW = [Var_b|Var_tb],
	Result0 = TmpDmntrW, Result1 = TmpGwiWWW.
merge([], Var_x, Var_x).
merge(Var_x, [], Result0) :- length(Var_x, TmpSKrTNs),
	TmpSKrTNs > 0, Result0 = Var_x.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a =< Var_b, TmpMbEaWz = [Var_b|Var_bs],
	merge(Var_as, TmpMbEaWz, TmpsbQGQq),
	TmphsSHro = [Var_a|TmpsbQGQq],
	Result0 = TmphsSHro.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a > Var_b, TmpdYVUqI = [Var_a|Var_as],
	merge(TmpdYVUqI, Var_bs, TmpjgOPtg),
	TmpFlURCd = [Var_b|TmpjgOPtg],
	Result0 = TmpFlURCd.
mergeSort([], []).
mergeSort([Var_x], [Var_x]).
mergeSort(Var_l, Result0) :- length(Var_l, TmphEZntf),
	TmphEZntf >= 2, divide(Var_l, Var_a, Var_b), mergeSort(Var_a, TmpjBeYkz),
	mergeSort(Var_b, TmpmNcgos),
	merge(TmpjBeYkz, TmpmNcgos, TmpCIHRiK),
	Result0 = TmpCIHRiK.
leaf(Result0) :- Result0 = Var_x, atom(Var_x), TmpgxwZwU = [],
	Var_x \== TmpgxwZwU.

