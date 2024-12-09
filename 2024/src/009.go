package main

import (
	"bufio"
	"fmt"
	"os"
)

const PATH = "../input/009.txt"

func main() {
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	var layout = []int{}
	var sectors = [][3]int{}
	var spaces = [][2]int{}

	get_layout(reader, &layout, &sectors, &spaces)
	layout_second := make([]int, len(layout))
	copy(layout_second, layout)

	back := len(layout) - 1
	// BRUTE

	for front := 0; front <= back; front++ {
		if layout[front] != -1 {
			continue
		}

		for {
			if layout[back] != -1 || back < front {
				break
			}
			back--
		}
		if back < front {
			break
		}
		layout[front] = layout[back]
		back--
	}

	total := 0
	for i, v := range layout[:back+1] {
		total += i * v
	}
	fmt.Println(total)

	// END BRUTE

	// SECOND

	for i := len(sectors) - 1; i >= 0; i-- {
		l := sectors[i][2] - sectors[i][1] + 1
		for ii, space := range spaces {
			if space[1] > sectors[i][1] {
				break
			}
			if space[0] < l {
				continue
			}
			for iii := sectors[i][1]; iii <= sectors[i][2]; iii++ {
				layout_second[iii] = -1
			}
			for iii := space[1]; iii < space[1]+l; iii++ {
				layout_second[iii] = sectors[i][0]
			}
			spaces[ii][0] -= l
			spaces[ii][1] += l

			break
		}
	}

	total_second := 0
	for i, v := range layout_second {
		if v < 0 {
			continue
		}
		total_second += i * v
	}
	fmt.Println(total_second)

}

func get_layout(reader *bufio.Reader, layout *[]int, sectors *[][3]int, spaces *[][2]int) {
	file_offset := 0
	sector_offset := 0
	for {

		line, prefix, err := reader.ReadLine()
		if err != nil {
			fmt.Println(err)
			panic("File error")
		}
		for i, b := range line {
			d := int(b) - 0x30

			for ii := 0; ii < d; ii++ {
				sector_offset++
				if (i+file_offset)%2 != 0 {
					*layout = append(*layout, -1)
				} else {
					*layout = append(*layout, (i+file_offset)/2)
				}
			}

			// build cache for the second part
			if d == 0 {
				continue
			}
			if (i+file_offset)%2 == 0 {
				sector := [3]int{(i + file_offset) / 2, sector_offset - d, sector_offset - 1}
				*sectors = append((*sectors), sector)
			} else {
				*spaces = append((*spaces), [2]int{d, sector_offset - d})
			}
		}
		file_offset += len(line)
		if !prefix {
			break
		}
	}
}
