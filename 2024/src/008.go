package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/008.txt"

var W = 0
var H = 0

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	var antennas = map[byte][][2]int{}
	y := 0

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		for x, c := range line {
			if c == byte('.') {
				continue
			}
			if _, exists := antennas[c]; exists {
				antennas[c] = append(antennas[c], [2]int{x, y})
			} else {
				antennas[c] = [][2]int{{x, y}}
			}
		}
		y++
		W = len(line)
	}

	H = y

	var visited = map[[2]int]bool{}
	var visited_p2 = map[[2]int]bool{}
	for _, a := range antennas {
		add_antinodes_from_freq_p1(&a, &visited)
		add_antinodes_from_freq_p2(&a, &visited_p2)
	}
	fmt.Println(len(visited))
	fmt.Println(len(visited_p2))
}

func add_antinodes_from_freq_p1(antennas *[][2]int, visited *map[[2]int]bool) {
	// obviously could skip duplicate pairs here
	for i := 0; i < len(*antennas); i++ {
		for ii := 0; ii < len(*antennas); ii++ {
			if i == ii {
				continue
			}
			a := (*antennas)[i]
			b := (*antennas)[ii]
			d := sub(b, a)
			for _, v := range [][2]int{add(b, d), sub(a, d)} {
				if is_on_board(v) {
					(*visited)[v] = true
				}
			}
		}
	}
}

func add_antinodes_from_freq_p2(antennas *[][2]int, visited *map[[2]int]bool) {
	// obviously could skip duplicate pairs here
	for i := 0; i < len(*antennas); i++ {
		for ii := 0; ii < len(*antennas); ii++ {
			if i == ii {
				continue
			}
			a := (*antennas)[i]
			b := (*antennas)[ii]
			d := sub(b, a)
			for _, v := range [][2]int{a, b} {
				for _, f := range []int{-1, 1} {
					iii := 1
					for {
						t := add(v, mul(d, iii*f))
						if !is_on_board(t) {
							break
						}
						(*visited)[t] = true
						iii++
					}
				}
			}
		}
	}
}

func is_on_board(pos [2]int) bool {
	return pos[0] >= 0 && pos[0] < W && pos[1] >= 0 && pos[1] < H
}

func add(a, b [2]int) [2]int {
	return [2]int{a[0] + b[0], a[1] + b[1]}
}
func sub(a, b [2]int) [2]int {
	return [2]int{a[0] - b[0], a[1] - b[1]}
}
func mul(a [2]int, i int) [2]int {
	return [2]int{a[0] * i, a[1] * i}
}
