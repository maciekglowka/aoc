let file = In_channel.open_text "../input/002.txt"
let line =  Option.get(In_channel.input_line file)

let range s =
  let n = String.split_on_char '-' s in
  (int_of_string (List.hd n), int_of_string(List.nth n 1))
  
let parse line =
  let ranges = String.split_on_char ',' line in
  List.map range ranges

let n_digits a =
  Float.to_int (Float.floor (Float.log10 (Float.of_int a))) + 1

let pow10 a =
  Float.to_int (Float.pow 10. (Float.of_int a))

let rec solve acc r =
  let a = fst r in
  let b = snd r in
  let n = n_digits a in
  let n = if n mod 2 = 0 then n else n + 1 in
  let t = pow10 (n / 2) in

  let a = if n_digits a mod 2 = 0 then a else pow10 (n - 1) in

  let base = a / t in
  let v = t * base + base in
  let next = t * (base + 1) + base + 1 in
  match v with
  | x when x > b -> acc
  | x when x < a -> solve acc (next, b)
  | _ -> v + solve acc (next, b)

let sum = List.fold_left solve 0 (parse line)
let () = Printf.printf "%d\n" sum

(* For a small number of digits a naive approach seams ok *)
let rec div n a = match a with
  | a when a > n -> []
  | a when n mod a = 0 -> a :: (div n (a + 1))
  | _ -> div n (a + 1)

let base a n d =
  let l = n / d in
  a / (pow10 (n - l))

let target base d l =
  let s acc v = acc + base * (pow10 v) in
  List.fold_left s 0 (List.init d (fun x -> l * x))

(* Dirty last minute hack to loose duplicates *)
let rec check_lower_d a n d =
  match d with
  | x when x < 2 -> 1
  | x when n mod d = 0 ->
    let bs = base a n d in
    if target bs d (n / d) = a then 0 else check_lower_d a n (d - 1)
  | _ -> check_lower_d a n (d - 1)

let rec solve2_single r n acc d =
  let a = fst r in
  let b = snd r in

  let bs = base a n d in
  let cur = target bs d (n / d) in
  let next = target (bs + 1) d (n / d) in

  (* let () = Printf.printf "Base: %d Cur: %d Next: %d \n" bs cur next in *)

  match cur with
  | x when x > b -> acc
  | x when x < a -> solve2_single (next, b) n acc d
  | _ -> (check_lower_d cur n (d - 1)) * cur + solve2_single (next, b) n acc d

let solve2_list r n ds =
  (* let () = Printf.printf "A: %d B: %d N: %d\n" (fst r) (snd r) n in *)
  List.fold_left (solve2_single r n) 0 ds

let rec solve2 acc r =
  let a = fst r in
  let b = snd r in
  let n = n_digits a in
  let ds = div n 2 in

  let mx = pow10 n in
  if b < mx then
    acc + solve2_list r n ds
  else
    acc + (solve2_list (a, mx - 1) n ds) + (solve2 0 (mx, b))

(* let () = Printf.printf "%d\n" (solve2 0 (222220, 222224)) *)
(* let () = Printf.printf "%d\n" (solve2_single (95, 115) 2 0 2) *)
(* let () = Printf.printf "%d\n" (solve2_list (1001, 1020) 4 [2; 4]) *)

let sum = List.fold_left solve2 0 (parse line)
let () = Printf.printf "%d\n" sum
