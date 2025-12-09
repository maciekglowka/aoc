let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let parse_line line =
  let parts = String.split_on_char ',' line in
  (int_of_string (List.hd parts), int_of_string (List.nth parts 1))

let parse_coords lines =
  let arr = Array.make (List.length lines) (0, 0) in
  let () = List.iteri (fun i l -> arr.(i) <- parse_line l) lines in
  arr

let coords = parse_coords (read_lines "../input/009.txt")
let count = Array.length coords
let manhattan a b = Int.abs (fst a - fst b) + Int.abs (snd a - snd b)

let rec get_dists coords i =
  let a = coords.(i) in
  let rec inner ii =
    if ii = count then []
    else
      let b = coords.(ii) in
      (manhattan a b, (a, b)) :: inner (ii + 1)
  in
  if i = count - 1 then [] else inner (i + 1) :: get_dists coords (i + 1)

let dists = List.flatten (get_dists coords 0)
let dists = List.sort (fun a b -> fst b - fst a) dists

let s =
  let p = List.hd dists in
  let a = fst (snd p) in
  let b = snd (snd p) in
  let dx = 1 + Int.abs (fst a - fst b) in
  let dy = 1 + Int.abs (snd a - snd b) in
  dx * dy

let () = Printf.printf "%d\n" s

let rec get_edges coords i vert hor =
  match i with
  | x when x = Array.length coords -> (vert, hor)
  | _ ->
      let a = coords.(i) in
      let bi = if i = Array.length coords - 1 then 0 else i + 1 in
      let b = coords.(bi) in
      if fst a = fst b then
        (* vertical *)
        let y0 = Int.min (snd a) (snd b) in
        let y1 = Int.max (snd a) (snd b) in
        let e = [| fst a; y0; y1 |] in
        get_edges coords (i + 1) (e :: vert) hor
      else
        (* horizontal *)
        let x0 = Int.min (fst a) (fst b) in
        let x1 = Int.max (fst a) (fst b) in
        let e = [| x0; x1; snd a |] in
        get_edges coords (i + 1) vert (e :: hor)

let edges = get_edges coords 0 [] []
let vert = fst edges
let hor = snd edges

(* sort by x *)
let vert = List.sort (fun a b -> a.(0) - b.(0)) vert

(* sort by y *)
let hor = List.sort (fun a b -> a.(2) - b.(2)) hor

let rec check_vert x y0 y1 left hor =
  let intersect x0 x1 y =
    if y <= y0 || y >= y1 then false
    else
      match x with
      | a when a = x0 -> left
      | a when a = x1 -> not left
      | a when a > x0 && a < x1 -> true
      | _ -> false
  in
  match hor with
  | [] -> false
  | h :: t ->
      let x0 = h.(0) in
      let x1 = h.(1) in
      let y = h.(2) in
      if y > y1 then false
      else if intersect x0 x1 y then true
      else check_vert x y0 y1 left t

let rec check_hor x0 x1 y top vert =
  let intersect x y0 y1 =
    if x <= x0 || x >= x1 then false
    else
      match y with
      | a when a = y0 -> top
      | a when a = y1 -> not top
      | a when a > y0 && a < y1 -> true
      | _ -> false
  in
  match vert with
  | [] -> false
  | h :: t ->
      let x = h.(0) in
      let y0 = h.(1) in
      let y1 = h.(2) in
      if x > x1 then false
      else if intersect x y0 y1 then true
      else check_hor x0 x1 y top t

let rec solve2 vert hor dists =
  match dists with
  | [] -> ((-1, -1), (-1, -1))
  | h :: t ->
      let a = fst (snd h) in
      let b = snd (snd h) in
      let x0 = Int.min (fst a) (fst b) in
      let x1 = Int.max (fst a) (fst b) in
      let y0 = Int.min (snd a) (snd b) in
      let y1 = Int.max (snd a) (snd b) in
      let left = check_vert x0 y0 y1 true hor in
      let right = check_vert x1 y0 y1 false hor in
      let top = check_hor x0 x1 y0 true vert in
      let bottom = check_hor x0 x1 y1 false vert in
      if left || right || top || bottom then solve2 vert hor t else (a, b)

let s2 =
  let p = solve2 vert hor dists in
  let a = fst p in
  let b = snd p in
  let dx = 1 + Int.abs (fst a - fst b) in
  let dy = 1 + Int.abs (snd a - snd b) in
  dx * dy

let () = Printf.printf "%d\n" s2
