package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	// "slices"
)

const PATH = "../input/021.txt"

var NUMERIC = map[byte][2]int{
	0:         [2]int{0, 0},
	byte('0'): [2]int{1, 0},
	byte('A'): [2]int{2, 0},
	byte('1'): [2]int{0, 1},
	byte('2'): [2]int{1, 1},
	byte('3'): [2]int{2, 1},
	byte('4'): [2]int{0, 2},
	byte('5'): [2]int{1, 2},
	byte('6'): [2]int{2, 2},
	byte('7'): [2]int{0, 3},
	byte('8'): [2]int{1, 3},
	byte('9'): [2]int{2, 3},
}
var DIRECTIONAL = map[byte][2]int{
	byte('<'): [2]int{0, 0},
	byte('v'): [2]int{1, 0},
	byte('>'): [2]int{2, 0},
	0:         [2]int{0, 1},
	byte('^'): [2]int{1, 1},
	byte('A'): [2]int{2, 1},
}
var CACHE = []map[[2]byte][][]byte{}

const LEVELS = 3

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	total := 0
	for i := 0; i < LEVELS; i++ {
		CACHE = append(CACHE, map[[2]byte][][]byte{})
	}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		fmt.Println(line)
		s := get_sequence(&line, 0)
		// print(&s)
		fmt.Println(len(s[0]), get_numeric_part(&line))
		c := len(s[0]) * get_numeric_part(&line)
		total += c
	}
	fmt.Println(total)
	// s := get_sequence(&[]byte{byte('0'), byte('2'), byte('9'), byte('A')}, 0)
	// // s := get_sequence(&[]byte{byte('0'), byte('2')}, 0)
	// // s := get_sequence(&[]byte{byte('0')}, 0)
	// print(&s)
	// fmt.Println(len(s[0]) * get_numeric_part(&[]byte{byte('0'), byte('2'), byte('9'), byte('A')}))
	// fmt.Println(get_numeric_part(&[]byte{byte('0'), byte('2'), byte('9'), byte('A')}))
	// get_sequence(&[]byte{byte('v')}, 2)
}

func get_numeric_part(pattern *[]byte) int {
	val := 0
	d := 0
	for i := len(*pattern) - 1; i >= 0; i-- {
		if (*pattern)[i] > 0x39 {
			continue
		}
		val += (int(math.Pow10(d)) * int((*pattern)[i]-0x30))
		d++
	}
	return val
}

// func get_sequence(keys *[]byte, level int) [][]byte {
// 	k := make([]byte, len(*keys))
// 	copy(k, *keys)
// 	k = append([]byte{byte('A')}, k...)

// 	var output = [][]byte{}

// 	for i := 0; i < len(k)-1; i++ {
// 		start := NUMERIC[k[i]]
// 		end := NUMERIC[k[i+1]]
// 		avoid := NUMERIC[0]

// 		if level > 0 {
// 			start = DIRECTIONAL[k[i]]
// 			end = DIRECTIONAL[k[i+1]]
// 			avoid = DIRECTIONAL[0]
// 		} else {

// 			// fmt.Printf("00000000000000000 %c %c\n", k[i], k[i+1])
// 		}

// 		s := get_moves(start, end, avoid)

// 		if level == 2 {
// 			// fmt.Printf("11111111111111111 %c %c\n", k[i], k[i+1])
// 			// print(&s)
// 			// fmt.Println("11111111111111111")
// 			if len(output) == 0 {
// 				output = s
// 			} else {
// 				var no = [][]byte{}
// 				for _, o := range output {
// 					for _, ss := range s {
// 						oo := make([]byte, len(o))
// 						copy(oo, o)
// 						oo = append(oo, ss...)
// 						no = append(no, oo)
// 					}
// 				}
// 				output = no
// 			}
// 			continue
// 		}

// 		var sss = [][]byte{}
// 		// cache_key := [2]byte{k[i], k[i+1]}
// 		// if cached, exists := CACHE[level][cache_key]; exists {
// 		// 	fmt.Println("Found cache", level, cache_key)
// 		// 	sss = cached
// 		// } else {
// 		for _, ss := range s {
// 			sss = append(sss, get_sequence(&ss, level+1)...)
// 		}
// 		// 	CACHE[level][cache_key] = sss
// 		// }

// 		if len(output) == 0 {
// 			output = sss
// 		} else {
// 			var no = [][]byte{}
// 			for _, o := range output {
// 				for _, ss := range sss {
// 					oo := make([]byte, len(o))
// 					copy(oo, o)
// 					oo = append(oo, ss...)
// 					no = append(no, oo)
// 				}
// 			}
// 			output = no
// 		}
// 	}

// 	if level == 0 {
// 		// that's really bad
// 		var no = [][]byte{}
// 		l := -1
// 		for _, o := range output {
// 			if l == -1 || len(o) < l {
// 				l = len(o)
// 			}
// 		}

// 		for _, o := range output {
// 			if len(o) == l {
// 				no = append(no, o)
// 			}
// 		}
// 		output = no
// 	}
// 	return output
// }

func get_sequence(keys *[]byte, level int) [][]byte {
	k := make([]byte, len(*keys))
	copy(k, *keys)
	if level == LEVELS {
		// if k[0] == byte('<') {
		// fmt.Println("--", string(k))
		// }
		return [][]byte{k}
	}

	k = append([]byte{byte('A')}, k...)

	var moves = [][]byte{}

	// fmt.Println("----", level)
	for i := 0; i < len(k)-1; i++ {
		start := NUMERIC[k[i]]
		end := NUMERIC[k[i+1]]
		avoid := NUMERIC[0]

		if level > 0 {
			start = DIRECTIONAL[k[i]]
			end = DIRECTIONAL[k[i+1]]
			avoid = DIRECTIONAL[0]
		}

		// fmt.Pr
		m := get_moves(start, end, avoid)
		// print(&m)

		if len(moves) == 0 {
			moves = m
		} else {
			var new_moves = [][]byte{}
			for _, prev := range moves {
				for _, next := range m {
					mm := make([]byte, len(prev))
					copy(mm, prev)
					mm = append(mm, next...)
					new_moves = append(new_moves, mm)
				}
			}
			moves = new_moves
		}
		// fmt.Println(moves)
	}

	// if level == 3 {
	// 	for _, m := range moves {
	// 		fmt.Println("m", string(m))
	// 	}
	// }

	var best = [][]byte{}
	for _, m := range moves {
		s := get_sequence(&m, level+1)
		for _, ss := range s {
			if len(best) == 0 {
				best = append(best, ss)
				continue
			}
			if len(best[0]) < len(ss) {
				continue
			}
			if len(best[0]) == len(ss) {
				best = append(best, ss)
			}
			best = [][]byte{ss}
		}
		// if len(best) == 0 || len(best) > len(s) {
		// 	best = s
		// }
		// if len(best) == len(s) && level != 3 {
		// 	fmt.Println("EQ", level)
		// }
	}
	// fmt.Println("--", level)
	// print(&best)
	return best
}

func get_moves(start, end, avoid [2]int) [][]byte {
	dx := clamp(end[0] - start[0])
	dy := clamp(end[1] - start[1])
	// fmt.Println("         dx dy", dx, dy, start, end)
	if dx == 0 && dy == 0 {
		return [][]byte{[]byte{byte('A')}}
	}

	var moves = [][]byte{}

	if dy != 0 {
		n := [2]int{start[0], start[1] + dy}
		var moves_v = [][]byte{}
		if n != avoid {
			moves_v = get_moves(n, end, avoid)
		}
		c := byte('^')
		if dy < 0 {
			c = byte('v')
		}
		for _, m := range moves_v {
			m = append([]byte{c}, m...)
			moves = append(moves, m)
		}
	}
	if dx != 0 {
		n := [2]int{start[0] + dx, start[1]}
		var moves_h = [][]byte{}
		if n != avoid {
			moves_h = get_moves(n, end, avoid)
		}
		c := byte('>')
		if dx < 0 {
			c = byte('<')
		}
		for _, m := range moves_h {
			m = append([]byte{c}, m...)
			moves = append(moves, m)
		}
	}
	return moves
}

func clamp(i int) int {
	if i == 0 {
		return 0
	}
	return i / abs(i)
}
func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func print(paths *[][]byte) {
	fmt.Println("----")
	for _, r := range *paths {
		for _, c := range r {
			fmt.Printf("%c", c)
		}
		fmt.Println("")
	}
	fmt.Println("")

}

// func print(path *[]byte) {
// 	fmt.Println("----")
// 	for _, c := range *path {
// 		fmt.Printf("%c", c)
// 	}
// 	fmt.Println("")
// 	fmt.Println("")
// }
