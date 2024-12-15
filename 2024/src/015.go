package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/015.txt"
const MAX = 255

const WALL = byte('#')
const PC = byte('@')
const BOX = byte('O')
const BOX_L = byte('[')
const BOX_R = byte(']')
const FLOOR = byte('.')

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	var board [MAX][MAX]byte
	var board_2 [MAX][MAX]byte

	h := 0
	// w := 0
	p := [2]int{-1, -1}
	p2 := [2]int{-1, -1}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		if len(line) == 0 {
			break
		}
		// w = 2 * len(line)

		for x, c := range line {
			board[h][x] = c
			if c == PC {
				p = [2]int{x, h}
			}
		}

		for x, c := range line {
			switch c {
			case WALL:
				board_2[h][2*x] = WALL
				board_2[h][2*x+1] = WALL
			case BOX:
				board_2[h][2*x] = BOX_L
				board_2[h][2*x+1] = BOX_R
			case PC:
				board_2[h][2*x] = PC
				board_2[h][2*x+1] = FLOOR
				p2 = [2]int{2 * x, h}
			case FLOOR:
				board_2[h][2*x] = FLOOR
				board_2[h][2*x+1] = FLOOR
			}
		}

		h++
	}

	var moves = [][2]int{}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		for _, c := range line {
			d := [2]int{0, 0}
			switch c {
			case byte('<'):
				d = [2]int{-1, 0}
			case byte('>'):
				d = [2]int{1, 0}
			case byte('^'):
				d = [2]int{0, -1}
			case byte('v'):
				d = [2]int{0, 1}
			}

			moves = append(moves, d)
		}
	}

	for _, d := range moves {
		if move(p, d, &board, false) {
			p = add(p, d)
		}
	}

	total := 0
	for y, line := range board {
		for x, c := range line {
			if c == BOX {
				total += 100*y + x
			}
		}
	}
	fmt.Println(total)

	bak := board_2
	for _, d := range moves {
		bak = board_2
		if move(p2, d, &board_2, false) {
			p2 = add(p2, d)
		} else {
			board_2 = bak
		}
		// print(&board_2, w, h)
	}
	total_2 := 0
	for y, line := range board_2 {
		for x, c := range line {
			if c == BOX_L {
				total_2 += 100*y + x
			}
		}
	}
	fmt.Println(total_2)
}

func move(p [2]int, d [2]int, board *[MAX][MAX]byte, secondary bool) bool {
	n := add(p, d)
	can := false
	switch (*board)[n[1]][n[0]] {
	case FLOOR:
		can = true
	case WALL:
		can = false
	case BOX, BOX_L, BOX_R:
		can = move(n, d, board, false)
	}
	if can && (*board)[p[1]][p[0]] == BOX_L && d[1] != 0 && !secondary {
		can = can && move([2]int{p[0] + 1, p[1]}, d, board, true)
	}
	if can && (*board)[p[1]][p[0]] == BOX_R && d[1] != 0 && !secondary {
		can = can && move([2]int{p[0] - 1, p[1]}, d, board, true)
	}
	if can {
		(*board)[n[1]][n[0]] = (*board)[p[1]][p[0]]
		(*board)[p[1]][p[0]] = FLOOR
	}
	return can
}

func add(a, b [2]int) [2]int {
	return [2]int{a[0] + b[0], a[1] + b[1]}
}

func print(board *[MAX][MAX]byte, w, h int) {
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			fmt.Printf("%c", (*board)[y][x])
		}
		fmt.Println()
	}
}
