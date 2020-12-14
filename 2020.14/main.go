package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	ins, err := ReadInstructions(f)
	check(err)

	sum := ins.Sum()
	fmt.Println("Memory Sum:", sum)
}

type Instruction struct {
	Mask    string
	Address int
	Value   int
}
type Instructions []Instruction

func (is Instructions) Sum() (sum int) {
	mem := make(map[int]int)
	for _, i := range is {
		mem[i.Address] = i.Result()
	}
	for _, v := range mem {
		sum+=v
	}
	return
}

func (i Instruction) Result() (int) {
	var val int
	for p := 0; p < len(i.Mask); p++ {
		mChar := i.Mask[len(i.Mask)-p-1]
		if mChar == 'X' {
			val |= i.Value & TwoPow(p)
		}
		if mChar == '1' {
			val |= TwoPow(p)
		}
	}
	return val
}

func TwoPow(n int) int {
	result := 1
	for exp := 1; exp <= n; exp++ {
		result *= 2
	}
	return result
}

func ReadInstructions(r io.Reader) (ins Instructions, err error) {
	scnr := bufio.NewScanner(r)
	var mask string
	for scnr.Scan() {
		sides := strings.Split(strings.TrimSpace(scnr.Text()), " = ")
		if sides[0] == "mask" {
			mask = sides[1]
			continue
		}
		var a, v int
		_, err = fmt.Sscanf(sides[0], "mem[%d]", &a)
		if err != nil {
			return
		}
		v, err = strconv.Atoi(sides[1])
		if err != nil {
			return
		}
		ins = append(ins, Instruction{Mask: mask, Address: a, Value: v})
	}
	err = scnr.Err()
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected error: %+v", err)
	}
}
