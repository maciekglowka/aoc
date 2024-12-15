package main

import (
	"bufio"
	"fmt"
	"os"
	// "slices"
	"strconv"
	"strings"
)

const PATH = "../input/014.txt"
const ITERATIONS = 100
const W = 101
const H = 103

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	mx := W / 2
	my := H / 2
	var robots = [][4]int{}

	for {
		if !scanner.Scan() {
			break
		}
		line := scanner.Text()
		r := get_robot(&line)
		robots = append(robots, r)
	}

	// symmetry test
	// tl := [][2]int{[2]int{2, 1}, [2]int{0, 0}, [2]int{2, 1}}
	// tr := [][2]int{[2]int{4, 1}, [2]int{4, 1}, [2]int{6, 0}}
	// fmt.Println(cmp_quad(&tl, &tr, 3))

	totals := [4]int{0, 0, 0, 0}

	for _, r := range robots {

		x := clamp(r[0]+ITERATIONS*r[2], W)
		y := clamp(r[1]+ITERATIONS*r[3], H)

		if x == mx || y == my {
			continue
		}

		o := 0
		if x > mx {
			o++
		}
		if y > my {
			o += 2
		}
		totals[o]++
	}
	fmt.Println(totals[0] * totals[1] * totals[2] * totals[3])

	i := 0
	for {
		test := true
		for _, r := range robots {
			x := clamp(r[0]+i*r[2], W)
			y := clamp(r[1]+i*r[3], H)

			if y < -2*x+H-2 || y < 2*x-H+2 {
				test = false
				break
			}
		}
		if !test {
			i++
			continue
		}
		break
	}
	fmt.Println(i)
}

func get_robot(s *string) [4]int {
	parts := strings.Fields(*s)
	pos_s := strings.Split(parts[0], "=")[1]
	v_s := strings.Split(parts[1], "=")[1]

	pp := strings.Split(pos_s, ",")
	x, _ := strconv.Atoi(pp[0])
	y, _ := strconv.Atoi(pp[1])

	vp := strings.Split(v_s, ",")
	vx, _ := strconv.Atoi(vp[0])
	vy, _ := strconv.Atoi(vp[1])

	return [4]int{x, y, vx, vy}
}

func clamp(v int, top int) int {
	if v >= 0 {
		return v % top
	}
	m := -v % top
	if m == 0 {
		return 0
	}
	return top - m
}

func below_line(quad *[][2]int, my, ov int) bool {
	for _, p := range *quad {
		if p[1]-ov < -p[0]+my {
			return false
		}
	}
	return true
}

// func cmp_quad(left, right *[][2]int, mx int) bool {
// 	if len(*left) != len(*right) {
// 		return false
// 	}
// 	mirror_quad(right, mx)
// 	slices.SortStableFunc(*left, cmp)
// 	slices.SortStableFunc(*right, cmp)

//		for i, _ := range *left {
//			if cmp((*left)[i], (*right)[i]) != 0 {
//				return false
//			}
//		}
//		return true
//	}

func cmp_quad(left, right *[][2]int, mx int) bool {
	lm := make(map[[2]int]bool)
	rm := make(map[[2]int]bool)
	mirror_quad(right, mx)

	for _, p := range *left {
		lm[p] = true
	}
	for _, p := range *right {
		rm[p] = true
	}
	fmt.Println(lm, rm)

	for k, _ := range lm {
		if !rm[k] {
			return false
		}
	}
	return true
}

func mirror_quad(quad *[][2]int, mx int) {
	for i, _ := range *quad {
		(*quad)[i][0] = mx - ((*quad)[i][0] - mx)
	}
}

func cmp(a, b [2]int) int {
	if a[0] == b[0] {
		return b[1] - a[1]
	}
	return b[0] - a[0]
}
