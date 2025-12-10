#load "str.cma"

let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let diagram_re = Str.regexp {re|\[.+\]|re}
let buttons_re = Str.regexp {re|(\(\([0-9]+,?\)+\))|re}

let rec from_bits seq =
  match seq with
  | [] -> 0
  | h :: t ->
      let v = 1 lsl h in
      v lor from_bits t

let parse_diagram s =
  let chars = String.to_seq s in
  let bits =
    Seq.mapi (fun i c -> if c = '#' then Some (i - 1) else None) chars
  in
  let bits = List.of_seq (Seq.filter_map (fun a -> a) bits) in
  from_bits bits

let parse_button_seq s =
  let s = String.split_on_char ',' s in
  let bits = List.map (fun a -> int_of_string a) s in
  from_bits bits

let rec parse_buttons line i acc =
  try
    let _ = Str.search_forward buttons_re line i in
    let e = Str.match_end () in
    let s = Str.matched_group 1 line in
    let s = parse_button_seq s in
    parse_buttons line e (s :: acc)
  with Not_found -> acc

let parse_line line =
  let _ = Str.search_forward diagram_re line 0 in
  let diagram_str = Str.matched_string line in
  let diagram_bits = parse_diagram diagram_str in
  let button_bits = parse_buttons line 0 [] in
  (diagram_bits, button_bits)

let rec parse_input lines =
  match lines with [] -> [] | h :: t -> parse_line h :: parse_input t

let input = parse_input (read_lines "../input/010.txt")

let rec solve_step visited cur buttons i =
  let next =
    List.map
      (fun c ->
        List.filter_map
          (fun b ->
            let v = c lxor b in
            if Hashtbl.mem visited v then None
            else
              let () = Hashtbl.add visited v true in
              Some v)
          buttons)
      cur
  in

  match Hashtbl.find_opt visited 0 with
  | Some _ -> i
  | None -> solve_step visited (List.flatten next) buttons (i + 1)

let solve_single input =
  let diagram = fst input in
  let buttons = snd input in
  let visited = Hashtbl.create 128 in
  solve_step visited [ diagram ] buttons 1

let rec solve input =
  match input with [] -> 0 | h :: t -> solve_single h + solve t

let () = Printf.printf "%d\n" (solve input)
