type coord = { x : int; y : int }

let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let str_lines = read_lines "../input/007.txt"
let board = Hashtbl.create 128
let start = ref { x = 0; y = 0 }

let parse_line line tbl y =
  let parse_char x c =
    match c with
    | '^' -> Hashtbl.add tbl { x; y } true
    | 'S' -> start := { x; y }
    | _ -> ()
  in
  String.iteri parse_char line

let () = List.iteri (fun i l -> parse_line l board i) str_lines
let max_y = List.length str_lines

let solve board =
  let visited = Hashtbl.create 128 in
  let rec path c =
    if c.y >= max_y then ()
    else
      match Hashtbl.find_opt visited c with
      | Some _ -> ()
      | None -> (
          match Hashtbl.find_opt board c with
          | None -> path { x = c.x; y = c.y + 1 }
          | Some _ ->
              Hashtbl.add visited c true;
              path { x = c.x - 1; y = c.y + 1 };
              path { x = c.x + 1; y = c.y + 1 })
  in
  let () = path !start in
  Hashtbl.length visited

let s = solve board
let () = Printf.printf "%d\n" s

let solve2 board =
  let visited = Hashtbl.create 128 in
  let rec path c =
    if c.y >= max_y then 1
    else
      match Hashtbl.find_opt visited c with
      | Some v -> v
      | None -> (
          match Hashtbl.find_opt board c with
          | None -> path { x = c.x; y = c.y + 1 }
          | Some _ ->
              let v =
                path { x = c.x - 1; y = c.y + 1 }
                + path { x = c.x + 1; y = c.y + 1 }
              in
              let () = Hashtbl.add visited c v in
              v)
  in
  path !start

let s = solve2 board
let () = Printf.printf "%d\n" s
