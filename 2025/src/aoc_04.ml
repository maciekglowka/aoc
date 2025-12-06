type coord = { x : int; y : int }

module Coord = struct
  type t = coord

  let compare = compare
end

module GridMap = Map.Make (Coord)

let file = In_channel.open_text "../input/004.txt"

let print grid =
  let max_x =
    GridMap.fold (fun k v acc -> if k.x > acc then k.x else acc) grid 0
  in
  let max_y =
    GridMap.fold (fun k v acc -> if k.y > acc then k.y else acc) grid 0
  in
  for y = 0 to max_y do
    let () =
      for x = 0 to max_x do
        Printf.printf "%c"
          (match GridMap.find_opt { x; y } grid with
          | Some a -> a
          | None -> '.')
      done
    in
    Printf.printf "\n"
  done

let parse_row row y =
  let chars = Seq.mapi (fun i c -> (i, c)) (String.to_seq row) in
  let chars = Seq.filter (fun a -> snd a = '@') chars in
  GridMap.of_seq (Seq.map (fun a -> ({ x = fst a; y }, snd a)) chars)

let rec parse acc y =
  match In_channel.input_line file with
  | None -> acc
  | Some l ->
      parse GridMap.(union (fun k a b -> Some a) acc (parse_row l y)) (y + 1)

let input = parse GridMap.empty 0
(* let () = print input *)

let rec neighbours_x acc grid ox oy x y =
  match x with
  | a when a > 1 -> acc
  | _ ->
      let c =
        if Option.is_some (GridMap.find_opt { x = ox + x; y = oy + y } grid)
        then 1
        else 0
      in
      let c = if x = 0 && y = 0 then 0 else c in
      acc + c + neighbours_x acc grid ox oy (x + 1) y

let rec neighbours acc grid ox oy y =
  match y with
  | a when a > 1 -> acc
  | _ ->
      acc + neighbours_x 0 grid ox oy (-1) y + neighbours acc grid ox oy (y + 1)

let check_roll grid c = if neighbours 0 grid c.x c.y (-1) < 4 then 1 else 0
let solve grid = GridMap.fold (fun k _ acc -> acc + check_roll grid k) grid 0
let s = solve input
let () = Printf.printf "%d\n" s
let remove_step grid = GridMap.partition (fun k _ -> check_roll grid k = 0) grid

let rec remove_all grid =
  let next = remove_step grid in
  if GridMap.cardinal (snd next) = 0 then fst next else remove_all (fst next)

let solve2 grid =
  let a = GridMap.cardinal grid in
  let b = GridMap.cardinal (remove_all grid) in
  a - b

let s = solve2 input
let () = Printf.printf "%d\n" s
