divide([], [], []).
divide([Var_x], [Var_x], []).
divide([Var_a, Var_b|Var_tail], Result0, Result1) :- divide(Var_tail, Var_ta, Var_tb), TmpwHzWHZ = [Var_a|Var_ta],
	TmpGfPvJn = [Var_b|Var_tb],
	Result0 = TmpwHzWHZ, Result1 = TmpGfPvJn.
merge([], Var_x, Var_x).
merge(Var_x, [], Result0) :- length(Var_x, TmpMPVaHN),
	TmpMPVaHN > 0, Result0 = Var_x.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a =< Var_b, TmpFVzYFM = [Var_b|Var_bs],
	merge(Var_as, TmpFVzYFM, TmpfVIhxz),
	TmpjpGpED = [Var_a|TmpfVIhxz],
	Result0 = TmpjpGpED.
merge([Var_a|Var_as], [Var_b|Var_bs], Result0) :- Var_a > Var_b, TmpvvbXvv = [Var_a|Var_as],
	merge(TmpvvbXvv, Var_bs, TmppHIozM),
	TmpPtPQRY = [Var_b|TmppHIozM],
	Result0 = TmpPtPQRY.
mergeSort([], []).
mergeSort([Var_x], [Var_x]).
mergeSort(Var_l, Result0) :- length(Var_l, TmpiaMZFB),
	TmpiaMZFB >= 2, divide(Var_l, Var_a, Var_b), mergeSort(Var_a, TmpASZyoe),
	mergeSort(Var_b, TmpFCSUyS),
	merge(TmpASZyoe, TmpFCSUyS, TmpCyyHog),
	Result0 = TmpCyyHog.
leaf(Var_x) :- atom(Var_x), TmpUrdiPn = [],
	Var_x \== TmpUrdiPn.
flatten([], []).
flatten([Var_h|Var_tail], Result0) :- leaf(Var_h), flatten(Var_tail, TmpFvoiMY),
	TmpiUVIgZ = [Var_h|TmpFvoiMY],
	Result0 = TmpiUVIgZ.
flatten([Var_h|Var_tail], Result0) :- \+ leaf(Var_h), flatten(Var_h, Var_left), flatten(Var_tail, Var_right), append(Var_left, Var_right, TmpBIxlRw),
	Result0 = TmpBIxlRw.

