package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	instructions, err := ParseInstructions(f)
	check(err)

	acc1, err := instructions.Execute(0, 0, []int{})
	fmt.Printf("Looping program accumulator: %d, Error: %v\n", acc1, err)

	acc2, err := instructions.ModExecute()
	check(err)
	fmt.Printf("Modified program accumulator: %d\n", acc2)

}

type Instruction struct {
	Operation string
	Argument  int
}

type Instructions []Instruction

var instructionPattern = regexp.MustCompile(`^([a-z]{3}) ([+|-]\d+)$`)

type InstructionLoopError struct{}

func (ile InstructionLoopError) Error() string {
	return "Infinite Loop"
}

func ParseInstructions(r io.Reader) (Instructions, error) {
	scnr := bufio.NewScanner(r)
	var ins Instructions
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		matches := instructionPattern.FindStringSubmatch(line)
		if len(matches) != 3 {
			return ins, fmt.Errorf("bad instruction %s, matches: %+v", line, matches)
		}
		op := matches[1]
		arg, err := strconv.Atoi(matches[2])
		if err != nil {
			return ins, err
		}
		ins = append(ins, Instruction{op, arg})
	}
	err := scnr.Err()
	return ins, err
}

func (ins Instructions) Execute(idx int, acc int, ids []int) (int, error) {
	for _, id := range ids {
		if id == idx {
			return acc, InstructionLoopError{}
		}
	}
	ids = append(ids, idx)
	if idx >= len(ins) {
		return acc, nil
	}
	in := ins[idx]
	switch in.Operation {
	case "acc":
		return ins.Execute(idx+1, acc+in.Argument, ids)
	case "jmp":
		return ins.Execute(idx+in.Argument, acc, ids)
	case "nop":
		return ins.Execute(idx+1, acc, ids)
	default:
		return acc, fmt.Errorf("unknown command: %s", in.Operation)
	}
}

func (ins Instructions) ModExecute() (int, error) {
	for i, in := range ins {
		var newOp string
		if in.Operation == "nop" {
			newOp = "jmp"
		}
		if in.Operation == "jmp" {
			newOp = "nop"
		}
		if newOp != "" {
			newIns := make(Instructions, len(ins))
			copy(newIns, ins)
			newIns[i] = Instruction{newOp, in.Argument}
			acc, err := newIns.Execute(0, 0, []int{})
			if _, ok := err.(InstructionLoopError); ok {
				continue
			}
			return acc, err
		}
	}
	return 0, fmt.Errorf("couldn't make a valid program")
}

func check(err error) {
	if err != nil {
		log.Fatalln("Unexpected error:", err)
	}
}
