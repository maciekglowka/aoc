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
var CACHE = map[[3]byte]int{}

const LEVELS = 26

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	total := 0

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		s := get_total_length(&line)
		c := s * get_numeric_part(&line)
		total += c
	}
	fmt.Println(total)
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

func get_length(a, b byte, level int) int {
	if level == LEVELS {
		return 1
	}

	cache_key := [3]byte{byte(level), a, b}
	if cached, exists := CACHE[cache_key]; exists {
		return cached
	}

	start := DIRECTIONAL[a]
	end := DIRECTIONAL[b]
	avoid := DIRECTIONAL[0]
	move_arr := get_moves(start, end, avoid)

	best := -1

	for _, moves := range move_arr {
		count := 0
		moves = append([]byte{byte('A')}, moves...)
		for i := 0; i < len(moves)-1; i++ {
			count += get_length(moves[i], moves[i+1], level+1)
		}
		if best < 0 || best > count {
			best = count
		}
	}
	CACHE[cache_key] = best
	return best
}

func get_total_length(keys *[]byte) int {
	k := make([]byte, len(*keys))
	copy(k, *keys)

	k = append([]byte{byte('A')}, k...)

	count := 0

	for i := 0; i < len(k)-1; i++ {
		start := NUMERIC[k[i]]
		end := NUMERIC[k[i+1]]
		avoid := NUMERIC[0]

		move_arr := get_moves(start, end, avoid)
		best := -1

		for _, moves := range move_arr {
			moves = append([]byte{byte('A')}, moves...)
			c := 0
			for i := 0; i < len(moves)-1; i++ {
				c += get_length(moves[i], moves[i+1], 1)
			}
			if best < 0 || best > c {
				best = c
			}
		}

		count += best

	}
	return count
}

func get_moves(start, end, avoid [2]int) [][]byte {
	dx := clamp(end[0] - start[0])
	dy := clamp(end[1] - start[1])
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
