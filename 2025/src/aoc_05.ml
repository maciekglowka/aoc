type range = { a : int; b : int }

let combine r0 r1 =
  match r1 with
  | x when x.a > r0.b -> [ r0; r1 ]
  | x when x.b <= r0.b -> [ r0 ]
  | _ -> [ { a = r0.a; b = r1.b } ]

let combine_list l r = match l with [] -> [ r ] | h :: t -> combine r h @ t

let rec contains l v =
  match l with
  | [] -> 0
  | h :: t -> if h.a <= v && h.b >= v then 1 else contains t v

let file = In_channel.open_text "../input/005.txt"

let parse_range line =
  let values = String.split_on_char '-' line in
  let a = int_of_string (List.hd values) in
  let b = int_of_string (List.nth values 1) in
  { a; b }

let rec parse_range_section file acc =
  match Option.get (In_channel.input_line file) with
  | l when String.length l < 2 -> acc
  | l -> parse_range l :: parse_range_section file acc

let rec parse_value_section file acc =
  match In_channel.input_line file with
  | None -> acc
  | Some l -> int_of_string l :: parse_value_section file acc

let input_ranges = parse_range_section file []
let input_values = parse_value_section file []
let input_ranges = List.sort (fun r0 r1 -> r1.a - r0.a) input_ranges
let combined = List.fold_left combine_list [] input_ranges

let rec solve ranges values acc =
  match values with
  | [] -> acc
  | h :: t -> solve ranges t (acc + contains ranges h)

let () = Printf.printf "%d\n" (solve combined input_values 0)

let rec solve2 ranges acc =
  match ranges with [] -> acc | h :: t -> solve2 t (acc + h.b - h.a + 1)

let () = Printf.printf "%d\n" (solve2 combined 0)
