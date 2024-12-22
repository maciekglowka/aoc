package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

const PATH = "../input/022.txt"
const MOD = (1 << 24) - 1

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)
	total := 0

	total_seq := make(map[[4]int]int)

	for {

		line, _, err := reader.ReadLine()

		if line == nil || err != nil {
			break
		}
		val, _ := strconv.Atoi(string(line))

		seq := make(map[[4]int]int)
		total += fi(val, 2000, &seq)

		for k, v := range seq {
			if _, exists := total_seq[k]; exists {
				total_seq[k] += v
			} else {
				total_seq[k] = v
			}
		}
	}
	fmt.Println(total)

	best := 0
	for _, v := range total_seq {
		if v > best {
			best = v
		}
	}
	fmt.Println(best)
}

func fi(a, i int, seq *map[[4]int]int) int {
	window := [4]int{0, 0, 0, 0}
	for ii := 0; ii < i; ii++ {
		prev := a
		a = f(a)
		d := a%10 - prev%10
		v := a % 10

		if ii < 4 {
			window[ii] = d
			if ii < 3 {
				continue
			}
		} else {
			window[0] = window[1]
			window[1] = window[2]
			window[2] = window[3]
			window[3] = d
		}

		if _, exists := (*seq)[window]; !exists {
			(*seq)[window] = v
		}
		// fmt.Println(window)
		// fmt.Println(seq)
	}
	return a
}

func f(a int) int {
	a = (a ^ (a << 6)) & MOD
	a = (a ^ (a >> 5)) & MOD
	return (a ^ (a << 11)) & MOD
}
