package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const PATH = "../input/002.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	one := first(bufio.NewScanner(file))
	file.Seek(0, 0)
	two := second(bufio.NewScanner(file))
	fmt.Println(one)
	fmt.Println(two)
}

func first(scanner *bufio.Scanner) int {
	count := 0
	for scanner.Scan() {
		line := scanner.Text()
		levels := strings.Fields(line)

		if test_report(levels) {
			count++
		}
	}
	return count
}

func second(scanner *bufio.Scanner) int {
	count := 0
	for scanner.Scan() {
		line := scanner.Text()
		levels := strings.Fields(line)

		for i := 0; i < len(levels); i++ {
			var split []string
			split = append(split, levels[:i]...)
			split = append(split, levels[i+1:]...)
			if test_report(split) {
				count++
				break
			}
		}
	}
	return count
}

func test_report(levels []string) bool {
	dir := 0
	prev := -1

	for _, level := range levels {
		l, _ := strconv.Atoi(level)
		if prev == -1 {
			prev = l
			continue
		}

		d := l - prev
		prev = l
		ndir := normalize(d)

		if d == 0 || abs(d) > 3 {
			return false
		}
		if dir == 0 {
			dir = ndir
			continue
		}
		if dir != ndir {
			return false
		}
	}
	return true
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func normalize(i int) int {
	if i == 0 {
		return 0
	}
	return i / abs(i)
}
