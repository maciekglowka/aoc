package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

const PATH = "../input/020.txt"

var W = 0
var H = 0

var DIRS = [4][2]int{
	{0, 1},
	{1, 0},
	{0, -1},
	{-1, 0},
}

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	var board = [][]byte{}
	var field = [][]int{}

	start := [2]int{0, 0}
	end := [2]int{0, 0}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		W = len(line)
		board = append(board, make([]byte, len(line)))
		copy(board[H], line)

		field = append(field, slices.Repeat([]int{-1}, W))

		for x, c := range line {
			if c == byte('S') {
				start[0] = x
				start[1] = H
			}
			if c == byte('E') {
				end[0] = x
				end[1] = H
			}
		}

		H++
	}

	// fmt.Println(start, end)
	set_field(&board, &field, end)
	// print(&board)
	// print_field(&field)
	get_shortucts(&field, start, 2)
	get_shortucts(&field, start, 20)
}

func get_shortucts(field *[][]int, start [2]int, window int) {
	cur := start
	shortcuts := make(map[[4]int]int)
	over_100 := 0

	for {
		cost := (*field)[cur[1]][cur[0]]
		if cost == 0 {
			break
		}

		var q = [][2]int{cur}
		visited := make(map[[2]int]int)

		for {
			if len(q) == 0 {
				break
			}
			cc := q[0]
			q = q[1:]

			for _, d := range DIRS {
				n := cc
				n[0] += d[0]
				n[1] += d[1]
				if !is_on_board(n) {
					continue
				}

				dist := visited[cc] + 1
				if dist > window {
					continue
				}

				if _, exists := visited[n]; exists {
					if visited[n] <= dist {
						continue
					}
				}
				visited[n] = dist
				q = append(q, n)

				new_cost := (*field)[n[1]][n[0]]
				if new_cost == -1 {
					continue
				}

				val := cost - new_cost - dist
				if val < 2 {
					continue
				}
				if val >= 100 {
					over_100++
				}
				k := [4]int{cur[0], cur[1], n[0], n[1]}
				if _, exists := shortcuts[k]; exists {
					if shortcuts[k] < val {
						shortcuts[k] = val
					}
				} else {
					shortcuts[k] = val
				}
			}
		}

		for _, d := range DIRS {
			n := cur
			n[0] += d[0]
			n[1] += d[1]
			if !is_on_board(n) {
				continue
			}

			if (*field)[n[1]][n[0]] == cost-1 {
				cur = n
				break
			}
		}
	}
	fmt.Println(over_100)

	over_2 := make(map[int]int)
	total_2 := 0
	for _, v := range shortcuts {
		if v < 100 {
			continue
		}
		if _, exists := over_2[v]; exists {
			over_2[v] += 1
		} else {
			over_2[v] = 1
		}
		total_2 += 1
	}
	fmt.Println(total_2)
}

func set_field(board *[][]byte, field *[][]int, end [2]int) {
	// x, y, cost
	var q = [][2]int{end}
	(*field)[end[1]][end[0]] = 0

	for {
		if len(q) == 0 {
			break
		}
		cur := q[0]
		q = q[1:]

		for _, d := range DIRS {
			n := cur
			n[0] += d[0]
			n[1] += d[1]

			if !is_on_board([2]int{n[0], n[1]}) {
				continue
			}
			if (*board)[n[1]][n[0]] == byte('#') {
				continue
			}

			cost := (*field)[cur[1]][cur[0]] + 1

			if (*field)[n[1]][n[0]] >= 0 && (*field)[n[1]][n[0]] <= cost {
				continue
			}
			(*field)[n[1]][n[0]] = cost
			q = append(q, n)
		}
	}
}

func is_on_board(p [2]int) bool {
	return p[0] >= 0 && p[0] < W && p[1] >= 0 && p[1] < H
}
func print(board *[][]byte) {
	for y := range *board {
		for x := range (*board)[y] {
			fmt.Printf(" %c", (*board)[y][x])
		}
		fmt.Println()
	}
}

func print_field(field *[][]int) {
	for y := range *field {
		for x := range (*field)[y] {
			if (*field)[y][x] < 0 {
				fmt.Printf("  ")
			} else {
				fmt.Printf("%2d", (*field)[y][x])
			}
		}
		fmt.Println()
	}
}
