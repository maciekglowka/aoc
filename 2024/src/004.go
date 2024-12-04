package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/004.txt"
const MAX = 255

var DIRS = [][]int{
	{0, 1},
	{1, 0},
	{0, -1},
	{-1, 0},
	{-1, -1},
	{-1, 1},
	{1, -1},
	{1, 1},
}

var DIAGS = [][]int{
	{1, 1},
	{1, -1},
}
var XMAS = []byte{'X', 'M', 'A', 'S'}
var MAS = []byte{'M', 'A', 'S'}
var SAM = []byte{'S', 'A', 'M'}

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	board := make([][]byte, MAX)
	h := 0

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		board[h] = make([]byte, len(line))
		copy(board[h], line)
		h++
	}

	w := len(board[0])

	total := 0

	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if board[y][x] != XMAS[0] {
				continue
			}
			total += get_word_count_at(x, y, &board, h, w)
		}
	}
	fmt.Println(total)

	total_x := 0
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if board[y][x] != MAS[1] {
				continue
			}
			if has_x_mas_at(x, y, &board, h, w) {
				total_x += 1
			}
		}
	}
	fmt.Println(total_x)
}

func get_word_count_at(ox int, oy int, board *[][]byte, h int, w int) int {
	count := 0
	for _, d := range DIRS {
		for i := 1; i <= 3; i++ {
			x := ox + d[0]*i
			y := oy + d[1]*i
			if !is_on_board(x, y, h, w) {
				break
			}

			if (*board)[y][x] != XMAS[i] {
				break
			}
			if i == 3 {
				count++
			}
		}
	}
	return count
}

func has_x_mas_at(ox int, oy int, board *[][]byte, h int, w int) bool {
	for _, d := range DIAGS {
		word := []byte{0, 0, 0}
		for i := -1; i <= 1; i++ {
			x := ox + d[0]*i
			y := oy + d[1]*i
			if !is_on_board(x, y, h, w) {
				return false
			}
			word[i+1] = (*board)[y][x]
		}
		if !slice_cmp(word, MAS) && !slice_cmp(word, SAM) {
			return false
		}
	}
	return true
}

func is_on_board(x int, y int, h int, w int) bool {
	return x >= 0 && x < w && y >= 0 && y < h
}

func slice_cmp(a []byte, b []byte) bool {
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
