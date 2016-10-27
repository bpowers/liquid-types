let foldn =
  fun n ->
  fun b ->
  fun f ->
    let loop = (fix inner_loop ->
      fun i ->
      fun c ->
	if i < n then (inner_loop (i + 1) (f i c)) else c)
    in loop 0 b
in foldn 3 (3 > 2) 0
