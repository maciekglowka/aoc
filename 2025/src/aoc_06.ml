let file = In_channel.open_text "../input/006.txt"

let parse_values file =
  let line = Option.get (In_channel.input_line file) in
  let parts = String.split_on_char ' ' line in
  List.filter_map
    (fun a -> if String.length a > 0 then Some (int_of_string a) else None)
    parts

let parse_operators file =
  let line = Option.get (In_channel.input_line file) in
  let parts = String.split_on_char ' ' line in
  List.filter_map
    (fun a -> if String.length a > 0 then Some (String.get a 0) else None)
    parts

let values =
  parse_values file :: parse_values file :: parse_values file
  :: [ parse_values file ]

let operators = parse_operators file

let calculate_col vals op i =
  match op with
  | '+' -> List.fold_left (fun a v -> a + List.nth v i) 0 vals
  | '*' -> List.fold_left (fun a v -> a * List.nth v i) 1 vals
  | _ -> raise (Invalid_argument "Invalid operator!")

let results = List.mapi (fun i o -> calculate_col values o i) operators
let sum = List.fold_left ( + ) 0 results
let () = Printf.printf "%d\n" sum

(* Second *)
let () = In_channel.seek file 0L
let get_line file = Option.get (In_channel.input_line file)

let value_lines =
  get_line file :: get_line file :: get_line file :: [ get_line file ]

let rec parse_number l i acc =
  match l with [] -> acc | h :: t -> (h * i) + parse_number t (i * 10) acc

let parse_col lines i =
  let rec parse_digit l =
    let c = String.get l i in
    match Char.code c with
    | x when x > 48 && x < 58 -> Some (x - 48)
    | _ -> None
  in
  let digits = List.filter_map parse_digit lines in
  parse_number digits 1 0

let rec parse_section lines i acc =
  let v = try parse_col lines i with Invalid_argument _ -> 0 in
  match v with 0 -> acc | a -> a :: parse_section lines (i + 1) acc

let rec parse_sections lines ops op_i cur =
  let section = parse_section lines cur [] in
  let offset = List.length section in
  match op_i - List.length ops with
  | x when x >= 0 -> 0
  | _ -> (
      match List.nth ops op_i with
      | '+' ->
          List.fold_left ( + ) 0 section
          + parse_sections lines ops (op_i + 1) (cur + offset + 1)
      | '*' ->
          List.fold_left ( * ) 1 section
          + parse_sections lines ops (op_i + 1) (cur + offset + 1)
      | _ -> raise (Invalid_argument "Invalid operator!"))

let () = Printf.printf "%d\n" (parse_sections value_lines operators 0 0)
