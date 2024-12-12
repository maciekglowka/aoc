package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

const PATH = "../input/012.txt"

var DIRS = [4][2]int{
	{0, 1},
	{1, 0},
	{0, -1},
	{-1, 0},
}

var w = 0
var h = 0

type FK struct {
	o int
	d [2]int
}

type Area struct {
	fields    [][2]int
	perimeter int
	fences    map[FK][]int
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
	visited := make(map[[2]int]bool)
	var areas = []Area{}

	y := 0

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		board = append(board, make([]byte, len(line)))
		copy(board[y], line)
		y++
	}

	w = len(board[0])
	h = len(board)

	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			v := [2]int{x, y}
			_, ok := visited[v]
			if ok {
				continue
			}
			flood_area(v, &board, &areas, &visited)
		}
	}

	total := 0
	for _, a := range areas {
		total += a.perimeter * len(a.fields)
	}
	fmt.Println(total)

	total_edges := 0
	for _, a := range areas {
		edges := count_edges(&a.fences)
		total_edges += len(a.fields) * edges
	}
	fmt.Println(total_edges)
}

func count_edges(fences *map[FK][]int) int {
	count := 0
	for _, v := range *fences {
		sort.Sort(sort.IntSlice(v))

		last := -1000
		for _, a := range v {
			if a-last > 1 {
				count++
			}
			last = a
		}
	}
	return count
}

func flood_area(v [2]int, board *[][]byte, areas *[]Area, visited *map[[2]int]bool) {
	var q = [][2]int{v}
	area := Area{[][2]int{}, 0, map[FK][]int{}}

	for {
		if len(q) == 0 {
			break
		}
		cur := q[0]
		q = q[1:]
		_, ok := (*visited)[cur]
		if ok {
			continue
		}
		area.fields = append(area.fields, cur)
		(*visited)[cur] = true
		area_val := (*board)[cur[1]][cur[0]]

		for _, d := range DIRS {
			a := add(cur, d)
			if !is_on_board(a) {
				area.perimeter++
				add_fence(&area, cur, d)
				continue
			}
			val := (*board)[a[1]][a[0]]
			if val != area_val {
				area.perimeter++
				add_fence(&area, cur, d)
				continue
			}

			q = append(q, a)
		}
	}
	*areas = append(*areas, area)
}

func is_on_board(v [2]int) bool {
	return v[0] >= 0 && v[0] < w && v[1] >= 0 && v[1] < h
}

func add(a, b [2]int) [2]int {
	return [2]int{a[0] + b[0], a[1] + b[1]}
}

func add_fence(area *Area, cur [2]int, d [2]int) {
	k := FK{-1000, d}
	v := -1000
	if d[0] == 0 {
		k.o = cur[1]
		v = cur[0]
	} else {
		k.o = cur[0]
		v = cur[1]
	}

	if _, exists := area.fences[k]; exists {
		area.fences[k] = append(area.fences[k], v)
	} else {
		area.fences[k] = []int{v}
	}

}
