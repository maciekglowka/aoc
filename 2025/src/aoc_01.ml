let read_lines file_name =
  In_channel.with_open_text file_name In_channel.input_lines

let str_lines = read_lines "../input/001.txt"

let parse_line line =
  let len = String.length line in
  let sign = if line.[0] = 'L' then -1 else 1 in
  sign * int_of_string (String.sub line 1 (len - 1))

let input = List.map parse_line str_lines

let solve cur step =
  let n = (fst cur + step) mod 100 in
  let n = if n < 0 then 100 + n else n in
  let a = if n = 0 then snd cur + 1 else snd cur in
  (n, a)

let a = List.fold_left solve (50, 0) input
let () = Printf.printf "%d\n" (snd a)

let solve2 cur step =
  let v = fst cur + step in
  let n = v mod 100 in
  let a =
    if step > 0 then v / 100
    else if v > 0 then 0
    else if fst cur > 0 then 1 + (v / -100)
    else v / -100
  in
  let n = if n < 0 then 100 + n else n in
  (n, snd cur + a)

let a = List.fold_left solve2 (50, 0) input
let () = Printf.printf "%d\n" (snd a)
