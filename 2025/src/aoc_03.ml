let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let str_lines = read_lines "../input/003.txt"

let parse_line line =
  List.of_seq (Seq.map (fun c -> (Char.code c) - 48) (String.to_seq line))

let input = List.map parse_line str_lines

let rec solve cur_max first row = 
  match row with
  | [] -> cur_max
  | h :: t ->
    let n = 10 * first + h in
    let cm = if n > cur_max then n else cur_max in
    let f = if first > h then first else h in
    solve cm f t

let sum = List.fold_left (fun acc row -> acc + solve 0 0 row) 0 input
let () = Printf.printf "%d\n" sum

let rec replace_min v front back =
  match back with
  | [] -> front
  | h :: t ->
    if v < h then front @ back else replace_min h (front @ [v]) t

let rec solve2_row cur row =
  match row with
  | [] -> cur
  | h :: t ->
    let n = match h with
    | x when x >= List.hd cur -> replace_min h [] cur
    | _ -> cur
    in
    solve2_row n t

let solve2 row =
  let l = List.length row in
  let cur = List.drop (l - 12) row in
  let queue = List.rev (List.take (l - 12) row) in
  solve2_row cur queue

let rec value v row =
  match row with
  | [] -> v
  | h :: t -> value (10 * v + h) t
  
let s = List.fold_left (fun acc row -> acc + (value 0 (solve2 row))) 0 input
let () = Printf.printf "%d\n" s
