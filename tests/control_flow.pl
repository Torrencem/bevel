fbuzz(Var_x, Result0) :- (TmpXySKLG is Var_x  mod  3,
	TmpXySKLG =:= 0, TmpaPZVhL is Var_x  mod  2,
	TmpaPZVhL =:= 0, Result0 = fizzbuzz
  -> true
;  TmpAAmxwn is Var_x  mod  2,
	TmpAAmxwn =:= 0, Result0 = fizz
  -> true
;  TmpYJZcWx is Var_x  mod  3,
	TmpYJZcWx =:= 0, Result0 = buzz
  -> true
;  Result0 = Var_x).
pfbuzz(_) :- (between(1, 100, Var_x), fbuzz(Var_x, TmpfckiVH),
	writeln(TmpfckiVH), false
  -> true
;  true).

