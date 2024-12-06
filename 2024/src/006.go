package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/006.txt"
const MAX = 255

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

	pos := [2]int{-1, -1}
	dir := [2]int{0, -1}

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		board[h] = make([]byte, len(line))
		copy(board[h], line)
		h++

		if pos[0] == -1 {
			for i, b := range line {
				if b == byte('^') {
					pos[0] = i
					pos[1] = h - 1
				}
			}
		}
	}

	w := len(board[0])
	var visited = map[[2]int]bool{}

	for {
		if step(&board, &pos, &dir, w, h) {
			break
		}
		// fmt.Println(pos)
		visited[pos] = true
	}
	// fmt.Println(pos)
	fmt.Println(len(visited))
}

func step(board *[][]byte, pos *[2]int, dir *[2]int, w int, h int) bool {
	next := [2]int{(*pos)[0] + (*dir)[0], (*pos)[1] + (*dir)[1]}
	if !is_on_board(next, w, h) {
		return true
	}
	if (*board)[next[1]][next[0]] == byte('#') {
		// rotate
		*dir = [2]int{-(*dir)[1], (*dir)[0]}
	} else {
		*pos = next
	}
	return false
}

func is_on_board(pos [2]int, h int, w int) bool {
	return pos[0] >= 0 && pos[0] < w && pos[1] >= 0 && pos[1] < h
}
