let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let parse_line line =
  let parts = String.split_on_char ',' line in
  [|
    int_of_string (List.hd parts);
    int_of_string (List.nth parts 1);
    int_of_string (List.nth parts 2);
  |]

let coords lines =
  let arr = Array.make (List.length lines) [| 0; 0; 0 |] in
  let () = List.iteri (fun i l -> arr.(i) <- parse_line l) lines in
  arr

let target = 1000
let coords = coords (read_lines "../input/008.txt")
let count = Array.length coords

let count_dists i =
  let a = coords.(i) in
  let rec inner ii =
    match ii with
    | x when x = count -> []
    | _ ->
        let b = coords.(ii) in
        let dx = a.(0) - b.(0) in
        let dy = a.(1) - b.(1) in
        let dz = a.(2) - b.(2) in
        let d = (dx * dx) + (dy * dy) + (dz * dz) in
        [| d; i; ii |] :: inner (ii + 1)
  in
  inner (i + 1)

let rec dists i =
  match i with x when x = count -> [] | _ -> count_dists i @ dists (i + 1)

let dists = dists 0
let dists = List.sort (fun a b -> a.(0) - b.(0)) dists
let dists_trimmed = List.take target dists
let coord_circuits = Array.init count (fun i -> i)
let circuits = Array.init count (fun i -> [ i ])

let handle_pair coord_circuits circuits p =
  let a = p.(1) in
  let b = p.(2) in
  let ca = coord_circuits.(a) in
  let cb = coord_circuits.(b) in
  if ca = cb then ()
  else
    let () = List.iter (fun c -> coord_circuits.(c) <- ca) circuits.(cb) in
    let () = circuits.(ca) <- circuits.(ca) @ circuits.(cb) in
    circuits.(cb) <- []

let () = List.iter (handle_pair coord_circuits circuits) dists_trimmed
let counts = Array.map (fun a -> List.length a) circuits
let () = Array.sort (fun a b -> b - a) counts
let () = Printf.printf "%d\n" (counts.(0) * counts.(1) * counts.(2))
let coord_circuits = Array.init count (fun i -> i)
let circuits = Array.init count (fun i -> [ i ])

let rec solve2 coord_circuits circuits dists =
  match dists with
  | [] -> -1
  | h :: t ->
      let () = handle_pair coord_circuits circuits h in
      let i = coord_circuits.(h.(1)) in
      if List.length circuits.(i) = count then
        let x0 = coords.(h.(1)).(0) in
        let x1 = coords.(h.(2)).(0) in
        x0 * x1
      else solve2 coord_circuits circuits t

let () = Printf.printf "%d\n" (solve2 coord_circuits circuits dists)
