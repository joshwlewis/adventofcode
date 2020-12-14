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

	sumv1 := ins.SumV1()
	fmt.Println("Memory Sum ChipV1:", sumv1)

	sumv2 := ins.SumV2()
	fmt.Println("Memory Sum ChipV2:", sumv2)
}

type Instruction struct {
	Mask    string
	Address int
	Value   int
}
type Instructions []Instruction

func (is Instructions) SumV1() (sum int) {
	mem := make(map[int]int)
	for _, i := range is {
		mem[i.Address] = i.GetValue()
	}
	for _, v := range mem {
		sum+=v
	}
	return
}

func (is Instructions) SumV2() (sum int) {
	mem := make(map[int]int)
	for _, i := range is {
		for _, a := range i.GetAddresses() {
		  mem[a] = i.Value
		}
	}
	for _, v := range mem {
		sum+=v
	}
	return
}

func (i Instruction) GetValue() (int) {
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

func (i Instruction) GetAddresses() ([]int) {
	var address int
	var floaters []int
	for p := 0; p < len(i.Mask); p++ {
		mChar := i.Mask[len(i.Mask)-p-1]
		if mChar == 'X' {
			floaters = append(floaters, p)
		}
		if mChar == '1' {
			address |= TwoPow(p)
		}
		if mChar == '0' {
			address |= i.Address & TwoPow(p)
		}
	}
	addresses := []int{address}
	for _, f := range floaters {
		count := len(addresses)
		for i := 0; i < count; i++ {
			addresses = append(addresses, addresses[i] | TwoPow(f))
		}
	}
	return addresses
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
