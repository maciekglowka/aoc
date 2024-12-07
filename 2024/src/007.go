package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const PATH = "../input/007.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	total := 0
	total_2 := 0

	for {
		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}

		parts := strings.Split(string(line), ":")

		target, _ := strconv.Atoi(parts[0])

		factor_str := strings.Fields(parts[1])
		var factors = []int{}
		for _, s := range factor_str {
			v, _ := strconv.Atoi(s)
			factors = append(factors, v)
		}

		if check(target, &factors, false) {
			total += target
		}
		// could use first parts results in the second as well ;)
		if check(target, &factors, true) {
			total_2 += target
		}
	}
	fmt.Println(total)
	fmt.Println(total_2)
}

func check(target int, factors *[]int, with_con bool) bool {
	// could do early exiting for perf
	a := proc(add, 0, target, factors, 0, with_con)
	m := proc(mul, 0, target, factors, 0, with_con)

	if !with_con {
		return a || m
	}
	c := proc(con, 0, target, factors, 0, with_con)
	return a || m || c
}

func proc(f func(int, int) int, lhs int, target int, factors *[]int, i int, with_con bool) bool {
	// could do early exiting for perf
	if i == len(*factors) {
		return lhs == target
	}
	if lhs > target {
		return false
	}
	next := f(lhs, (*factors)[i])
	a := proc(add, next, target, factors, i+1, with_con)
	m := proc(mul, next, target, factors, i+1, with_con)
	if !with_con {
		return a || m
	}
	c := proc(con, next, target, factors, i+1, with_con)
	return a || m || c
}

func add(a int, b int) int {
	return a + b
}
func mul(a int, b int) int {
	return a * b
}

func con(a int, b int) int {
	if a == 0 {
		return b
	}
	return a*int(math.Pow10(int(math.Floor(math.Log10(float64(b))+1.)))) + b
}
