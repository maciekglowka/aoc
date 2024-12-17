package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"time"
)

const PATH = "../input/016.txt"

const WALL = byte('#')
const START = byte('S')
const END = byte('E')

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

	p := [2]int{-1, -1}
	e := [2]int{-1, -1}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		if len(line) == 0 {
			break
		}
		row := make([]byte, len(line))
		copy(row, line)
		board = append(board, row)

		for x, c := range line {
			if c == START {
				p[0] = x
				p[1] = len(board) - 1
			}
			if c == END {
				e[0] = x
				e[1] = len(board) - 1
			}
		}
	}
	// print(&board)
	best := find_path(p, e, &board)
	fmt.Println(best)
	ts := time.Now()
	fmt.Println(find_path_p2(p, e, &board, best), time.Since(ts))
}

func find_path(start, end [2]int, board *[][]byte) int {
	// x, y, dx, dy, cost
	var q = [][5]int{[5]int{start[0], start[1], 1, 0, 0}}
	var cost_so_far = map[[2]int]int{}

	for {
		slices.SortStableFunc(q, cmp)
		cur := q[0]
		q = q[1:]

		for _, d := range DIRS {
			n := cur
			n[0] += d[0]
			n[1] += d[1]
			n[2] = d[0]
			n[3] = d[1]

			cost := 1
			if [2]int{cur[2], cur[3]} != d {
				cost += 1000
			}
			n[4] += cost

			if prev, exists := cost_so_far[[2]int{n[1], n[0]}]; exists {
				if prev <= n[4] {
					continue
				}
			}

			cost_so_far[[2]int{n[1], n[0]}] = n[4]

			if [2]int{n[0], n[1]} == end {
				return n[4]
			}
			if (*board)[n[1]][n[0]] == WALL {
				continue
			}

			q = append(q, n)
		}
	}
}

type Path struct {
	nodes [][2]int
	front [2]int
	dir   [2]int
	cost  int
}

func find_path_p2(start, end [2]int, board *[][]byte, best int) int {
	var q = []Path{Path{[][2]int{}, start, [2]int{1, 0}, 0}}
	var best_so_far = map[[2]int]int{}

	var paths = []Path{}

	for {
		if len(q) == 0 {
			break
		}
		cur := q[0]
		q = q[1:]

		for _, d := range DIRS {
			if d[0] == -cur.dir[0] && d[1] == -cur.dir[1] {
				continue
			}
			n := cur.front
			n[0] += d[0]
			n[1] += d[1]

			if (*board)[n[1]][n[0]] == WALL {
				continue
			}

			new_cost := 1
			if cur.dir != d {
				new_cost += 1000
			}
			total_cost := cur.cost + new_cost

			if total_cost+manhattan(n, end) > best {
				continue
			}
			if prev, exists := best_so_far[n]; exists {
				if prev <= total_cost {
					// at worse be a single turn behind the best case scenario
					if total_cost-prev > 1000 {
						continue
					}
				} else {
					best_so_far[n] = total_cost
				}
			} else {
				best_so_far[n] = total_cost
			}

			nodes := make([][2]int, len(cur.nodes))
			copy(nodes, cur.nodes)
			nodes = append(nodes, n)

			new_path := Path{nodes, n, d, cur.cost + new_cost}
			if n == end {
				paths = append(paths, new_path)
			} else {

				q = append(q, new_path)
			}
		}
	}

	visited := map[[2]int]bool{}

	// backtrack from the end
	for _, path := range paths {

		for _, n := range path.nodes {
			visited[n] = true
		}
	}
	// fmt.Println(len(visited))
	// print(board, &visited)

	// +1 for start
	return len(visited) + 1
}

// func find_path_p2(start, end [2]int, board *[][]byte, best int) int {
// 	// x, y, dx, dy, cost
// 	var q = [][5]int{[5]int{start[0], start[1], 1, 0, 0}}
// 	var best_so_far = map[[2]int]int{}
// 	var came_from = map[[2]int][][2]int{}

// 	for {
// 		if len(q) == 0 {
// 			break
// 		}
// 		cur := q[0]
// 		q = q[1:]

// 		for _, d := range DIRS {
// 			if d[0] == -cur[2] && d[1] == -cur[3] {
// 				continue
// 			}
// 			n := cur
// 			n[0] += d[0]
// 			n[1] += d[1]
// 			n[2] = d[0]
// 			n[3] = d[1]

// 			if (*board)[n[1]][n[0]] == WALL {
// 				continue
// 			}

// 			cost := 1
// 			if [2]int{cur[2], cur[3]} != d {
// 				cost += 1000
// 			}
// 			n[4] += cost
// 			if n[4] > best {
// 				continue
// 			}

// 			pos := [2]int{n[0], n[1]}

// 			if prev, exists := best_so_far[pos]; exists {
// 				if prev <= n[4] {
// 					if prev+1000 < n[4] {
// 						continue
// 					}
// 				} else {
// 					best_so_far[pos] = n[4]
// 				}

// 				contains := false
// 				for _, vv := range came_from[pos] {
// 					if vv == [2]int{cur[0], cur[1]} {
// 						contains = true
// 					}
// 				}
// 				if !contains {
// 					came_from[pos] = append(came_from[pos], [2]int{cur[0], cur[1]})
// 				}

// 			} else {
// 				best_so_far[pos] = n[4]
// 				came_from[pos] = [][2]int{[2]int{cur[0], cur[1]}}
// 			}

// 			if pos == end {
// 				continue
// 			}

// 			q = append(q, n)

// 		}
// 	}

// 	visited := map[[2]int]bool{}

// 	// backtrack from the end
// 	var qv = [][2]int{end}

// 	for {
// 		if len(qv) == 0 {
// 			break
// 		}
// 		cur := qv[0]
// 		qv = qv[1:]
// 		visited[cur] = true

// 		// fmt.Println(came_from[cur])
// 		for _, v := range came_from[cur] {
// 			qv = append(qv, v)
// 		}
// 	}

// 	// fmt.Println(len(visited))
// 	print(board, &visited)

// 	// +1 for start
// 	return len(visited)
// }

func manhattan(a, b [2]int) int {
	return abs(b[0]-a[0]) + abs(b[1]-a[1])
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func print(board *[][]byte, visited *map[[2]int]bool) {
	for y := 0; y < len(*board); y++ {
		for x := 0; x < len((*board)[y]); x++ {
			c := (*board)[y][x]
			if _, ok := (*visited)[[2]int{x, y}]; ok {
				c = '^'
			}
			fmt.Printf("%c", c)
		}
		fmt.Println()
	}
}

func cmp(a, b [5]int) int {
	if a[4] < b[4] {
		return -1
	}
	return 1
}
