package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/010.txt"

var DIRS = [4][2]int{
	{0, 1},
	{1, 0},
	{0, -1},
	{-1, 0},
}

var w = 0
var h = 0

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	var board = [][]int{}
	heads := make(map[[2]int]bool)

	y := 0

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		row := []int{}
		for x, b := range line {
			row = append(row, int(b-0x30))
			if b-0x30 == 0 {
				heads[[2]int{x, y}] = true
			}
		}
		board = append(board, row)
		y++
	}

	w = len(board[0])
	h = len(board)

	// fmt.Println(board[:h], w, h)

	total := 0
	total_rating := 0
	for k, _ := range heads {
		v := get_head_score(&board, k[0], k[1])
		total += v[0]
		total_rating += v[1]
	}
	fmt.Println(total, total_rating)
}

func get_head_score(board *[][]int, x, y int) [2]int {
	var q = [][2]int{[2]int{x, y}}
	tops := make(map[[2]int]bool)
	rating := 0

	for {
		if len(q) == 0 {
			break
		}
		cur := q[0]
		cur_val := (*board)[cur[1]][cur[0]]
		q = q[1:]
		for _, d := range DIRS {
			v := add(cur, d)
			if !is_on_board(v) {
				continue
			}
			val := (*board)[v[1]][v[0]]
			if val-cur_val != 1 {
				continue
			}
			if val == 9 {
				rating++
				tops[v] = true
				continue
			}
			q = append(q, v)
		}
	}

	return [2]int{len(tops), rating}
}

func is_on_board(v [2]int) bool {
	return v[0] >= 0 && v[0] < w && v[1] >= 0 && v[1] < h
}

func add(a, b [2]int) [2]int {
	return [2]int{a[0] + b[0], a[1] + b[1]}
}
