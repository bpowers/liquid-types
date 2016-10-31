let foldn = (fun n -> fun b -> fun f ->
             (let rec loop = (fun i -> fun c ->
                              if i < n then (loop (i + 1) (f i c)) else c)
              in loop 0 b))
in foldn 3 (3 > 2) (fun i -> fun b -> b)
