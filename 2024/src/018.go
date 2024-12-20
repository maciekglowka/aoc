package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

const PATH = "../input/018.txt"
const DIM = 71
const TIME = 1024

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

	var drops = [][2]int{}

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		parts := strings.Split(string(line), ",")
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		drops = append(drops, [2]int{x, y})
	}

	obstacles := make(map[[2]int]bool)
	path := make(map[[2]int]bool)
	for i := 0; i < TIME; i++ {
		obstacles[drops[i]] = true
	}
	fmt.Println(find_path(&obstacles, &path))
	print(&obstacles, &path)
	fmt.Println("----")

	i := TIME
	for {
		path := make(map[[2]int]bool)
		obstacles[drops[i]] = true
		if find_path(&obstacles, &path) == -1 {
			break
		}
		i++
	}
	fmt.Println(drops[i])
}

func find_path(obstacles *map[[2]int]bool, path *map[[2]int]bool) int {
	// x, y, cost
	var q = [][3]int{[3]int{0, 0, 0}}
	end := [2]int{DIM - 1, DIM - 1}
	var best [DIM][DIM]int
	var came_from [DIM][DIM][2]int
	found := false

	for {
		if len(q) == 0 {
			return -1
		}
		slices.SortStableFunc(q, cmp)
		cur := q[0]
		q = q[1:]

		for _, d := range DIRS {
			n := cur
			n[0] += d[0]
			n[1] += d[1]
			n[2] = cur[2] + 1

			if !is_on_board([2]int{n[0], n[1]}) {
				continue
			}
			if _, ok := (*obstacles)[[2]int{n[0], n[1]}]; ok {
				continue
			}

			if best[n[0]][n[1]] != 0 && best[n[0]][n[1]] <= n[2] {
				continue
			}
			best[n[0]][n[1]] = n[2]
			came_from[n[0]][n[1]] = [2]int{cur[0], cur[1]}

			if [2]int{n[0], n[1]} == end {
				found = true
				// return n[2]
			}
			q = append(q, n)
		}
		if found {
			break
		}
	}

	cur := end
	for {
		(*path)[cur] = true
		if cur[0] == 0 && cur[1] == 0 {
			break
		}
		cur = came_from[cur[0]][cur[1]]
	}
	return len(*path) - 1
}

func is_on_board(p [2]int) bool {
	return p[0] >= 0 && p[0] < DIM && p[1] >= 0 && p[1] < DIM
}

func print(obstacles *map[[2]int]bool, path *map[[2]int]bool) {
	for y := 0; y < DIM; y++ {
		for x := 0; x < DIM; x++ {
			c := '.'
			if _, ok := (*obstacles)[[2]int{x, y}]; ok {
				c = '#'
			}
			if _, ok := (*path)[[2]int{x, y}]; ok {
				c = 'O'
			}
			fmt.Printf("%c", c)
		}
		fmt.Println()
	}
}

func cmp(a, b [3]int) int {
	if a[2] < b[2] {
		return -1
	}
	return 1
}
