package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const PATH = "../input/017.txt"

type CPU struct {
	A    int
	B    int
	C    int
	code []int
	pc   int
	out  []int
}

func cpu_new(code []int) CPU {
	return CPU{0, 0, 0, code, 0, []int{}}
}

func read_output(cpu *CPU) string {
	var s = []string{}
	for _, n := range (*cpu).out {
		s = append(s, strconv.Itoa(n))
	}
	return strings.Join(s, ",")
}

func asm(s string) []int {
	byte_str := strings.Split(s, ",")
	var bytes = []int{}
	for _, s := range byte_str {
		n, _ := strconv.Atoi(s)
		bytes = append(bytes, n)
	}
	return bytes
}

func disasm(code *[]int) {
	for i := 0; i < len(*code); i += 2 {
		op := ""
		switch (*code)[i] {
		case 0:
			op = "adv"
		case 1:
			op = "bxl"
		case 2:
			op = "bst"
		case 3:
			op = "jnz"
		case 4:
			op = "bxc"
		case 5:
			op = "out"
		case 6:
			op = "bdv"
		case 7:
			op = "cdv"
		}
		fmt.Println(i, ":", op, (*code)[i+1])
	}
}

func main() {
	fmt.Println("Running PRE stage tests....")
	pre_tests()

	fmt.Println("Loading cartridge from", PATH)
	file, err := os.Open(PATH)
	if err != nil {
		fmt.Println("Error", err)
		return
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	line, _, _ := reader.ReadLine()
	parts_a := strings.Split(string(line), ":")
	a, _ := strconv.Atoi(parts_a[1][1:])

	line, _, _ = reader.ReadLine()
	parts_b := strings.Split(string(line), ":")
	b, _ := strconv.Atoi(parts_b[1][1:])

	line, _, _ = reader.ReadLine()
	parts_c := strings.Split(string(line), ":")
	c, _ := strconv.Atoi(parts_c[1][1:])

	line, _, _ = reader.ReadLine()
	line, _, _ = reader.ReadLine()
	parts_p := strings.Split(string(line), ":")

	cpu := cpu_new(asm(parts_p[1][1:]))
	cpu.A = a
	cpu.B = b
	cpu.C = c

	// fmt.Println("Read", cpu)

	run(&cpu)
	fmt.Println("First:", read_output(&cpu))
	// disasm(&cpu.code)

	i := 0
	idx := len(cpu.code) - 1

	for {
		cpu := cpu_new(asm(parts_p[1][1:]))
		cpu.A = i
		cpu.B = b
		cpu.C = c

		run(&cpu)
		prev := true
		for ii := 0; ii+idx < len(cpu.code); ii++ {
			if cpu.out[ii] != cpu.code[idx+ii] {
				prev = false
				break
			}
		}
		if !prev {
			i++
			continue
		}

		idx--
		if idx < 0 {
			break
		}
		i *= 8
	}
	fmt.Println("Second:", i)
}

func run(cpu *CPU) {
	for {
		step(cpu)
		if cpu.pc >= len(cpu.code) {
			break
		}
		// fmt.Println(cpu.A, cpu.B, cpu.C)
	}
}

func step(cpu *CPU) {
	op_code := cpu.code[cpu.pc]
	operand := cpu.code[cpu.pc+1]
	// fmt.Println("Executing:", op_code, operand)

	switch op_code {
	case 0:
		// adv
		cpu.A = cpu.A / int(math.Pow(float64(2), float64(combo(operand, cpu))))
	case 1:
		// bxl
		cpu.B = cpu.B ^ operand
	case 2:
		// bst
		cpu.B = combo(operand, cpu) % 8
	case 3:
		// jnz
		if cpu.A != 0 {
			// fmt.Println("JMP")
			cpu.pc = operand
			cpu.pc -= 2
		}
	case 4:
		// bxc
		cpu.B = cpu.B ^ cpu.C
	case 5:
		// out
		cpu.out = append(cpu.out, combo(operand, cpu)%8)
	case 6:
		// bdv
		cpu.B = cpu.A / int(math.Pow(float64(2), float64(combo(operand, cpu))))
	case 7:
		// cdv
		cpu.C = cpu.A / int(math.Pow(float64(2), float64(combo(operand, cpu))))
	default:
		panic("Unknown OP_CODE")
	}

	cpu.pc += 2
}

func combo(operand int, cpu *CPU) int {
	switch operand {
	case 0, 1, 2, 3:
		return operand
	case 4:
		return cpu.A
	case 5:
		return cpu.B
	case 6:
		return cpu.C
	}
	panic("Invalid combo operand")
}

func pre_tests() {
	test_op_0()
	test_op_1()
	test_op_2()
	test_op_3()
	test_op_4()
	test_op_5()
	test_op_6()
	test_op_7()

	test_26()
	test_505154()
	test_015430()
	test_17()
	test_40()
}

// AOC example tests

func test_26() {
	code := []int{2, 6}
	cpu := cpu_new(code)
	cpu.C = 9
	run(&cpu)
	if cpu.B != 1 {
		panic("")
	}
}

func test_505154() {
	code := []int{5, 0, 5, 1, 5, 4}
	cpu := cpu_new(code)
	cpu.A = 10
	run(&cpu)
	if read_output(&cpu) != "0,1,2" {
		panic("")
	}
}

func test_015430() {
	code := []int{0, 1, 5, 4, 3, 0}
	cpu := cpu_new(code)
	cpu.A = 2024
	run(&cpu)
	if cpu.A != 0 {
		panic("")
	}
	if read_output(&cpu) != "4,2,5,6,7,7,7,7,3,1,0" {
		panic("")
	}
}

func test_17() {
	code := []int{1, 7}
	cpu := cpu_new(code)
	cpu.B = 29
	run(&cpu)
	if cpu.B != 26 {
		panic("")
	}
}

func test_40() {
	code := []int{4, 0}
	cpu := cpu_new(code)
	cpu.B = 2024
	cpu.C = 43690
	run(&cpu)
	if cpu.B != 44354 {
		panic("")
	}
}

// OP_CODE tests

func test_op_0() {
	code := []int{0, 3}
	cpu := cpu_new(code)
	cpu.A = 16
	cpu.C = 9
	step(&cpu)
	if cpu.A != 16/8 {
		panic("")
	}

	code_2 := []int{0, 3}
	cpu_2 := cpu_new(code_2)
	cpu_2.A = 17
	cpu_2.C = 9
	step(&cpu_2)
	if cpu_2.A != 16/8 {
		panic("")
	}

	code_3 := []int{0, 5}
	cpu_3 := cpu_new(code_3)
	cpu_3.A = 16
	cpu_3.B = 4
	cpu_3.C = 9

	step(&cpu_3)
	if cpu_3.A != 1 {
		panic("")
	}
}

func test_op_1() {
	code := []int{1, 7}
	cpu := cpu_new(code)
	cpu.B = 9
	step(&cpu)
	if cpu.B != 14 {
		panic("")
	}

}

func test_op_2() {
	code := []int{2, 4}
	cpu := cpu_new(code)
	cpu.A = 11
	cpu.B = 9
	step(&cpu)
	if cpu.B != 3 {
		panic("")
	}
}

func test_op_3() {
	code := []int{3, 44}
	cpu := cpu_new(code)
	cpu.A = 11
	cpu.B = 9

	step(&cpu)
	if cpu.pc != 44 {
		panic("")
	}

	cpu_2 := cpu_new(code)
	cpu_2.B = 9
	step(&cpu_2)
	if cpu_2.pc != 2 {
		panic("")
	}
}

func test_op_4() {
	code := []int{4, -1}
	cpu := cpu_new(code)
	cpu.A = 2
	cpu.B = 9
	cpu.C = 7
	step(&cpu)
	if cpu.B != 14 {
		panic("")
	}
}

func test_op_5() {
	code := []int{5, 5}
	cpu := cpu_new(code)
	cpu.B = 11
	step(&cpu)
	if cpu.out[0] != 3 {
		panic("")
	}
}

func test_op_6() {
	code := []int{6, 6}
	cpu := cpu_new(code)
	cpu.A = 48
	cpu.C = 3
	step(&cpu)
	if cpu.B != 48/8 {
		panic("")
	}
}

func test_op_7() {
	code := []int{7, 6}
	cpu := cpu_new(code)
	cpu.A = 48
	cpu.C = 3
	step(&cpu)
	if cpu.C != 48/8 {
		panic("")
	}
}
