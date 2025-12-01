let read_lines file_name = In_channel.with_open_text file_name
  In_channel.input_lines

let str_lines = read_lines "001.txt"

let parse_line line = let a = String.split_on_char ' ' line
  in int_of_string (List.hd a), int_of_string(List.nth a 3)

let a, b =
  let pairs = List.map parse_line str_lines in
  List.split pairs

let sa, sb = 
  List.sort compare a, List.sort compare b

let diff = List.map2 (fun a b -> abs(a - b)) sa sb
let () = Printf.printf "First: %d\n" (List.fold_left (+) 0 diff)

let sim =
  let fi a = List.filter (fun b -> b = a) b in
  let score a = a * (List.length (fi a)) in
  List.map score a

let () = Printf.printf "Second: %d\n" (List.fold_left (+) 0 sim)
