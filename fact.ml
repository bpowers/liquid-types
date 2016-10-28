let rec factorial = fun x -> if x = 0 then 1 else x * (factorial (x - 1)) in
factorial 5
