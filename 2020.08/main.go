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

	xidx, pidx, acc, err := instructions.Execute(0, 0, []int{})
	check(err)

	fmt.Printf("After line %d, exited at line %d, with an accumulator at %d\n", pidx, xidx, acc)
}

type Instruction struct {
	Operation string
	Argument  int
}

type Instructions []Instruction

var instructionPattern = regexp.MustCompile(`^([a-z]{3}) ([+|-]\d+)$`)

func ParseInstructions(r io.Reader) (Instructions, error) {
	scnr := bufio.NewScanner(r)
	var ins Instructions
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		matches := instructionPattern.FindStringSubmatch(line)
		if len(matches) != 3 {
			return ins, fmt.Errorf("bad instruction %s. Matches: %+v.", line, matches)
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

func (ins Instructions) Execute(idx int, acc int, ids []int) (int, int, int, error) {
	for _, id := range ids {
		if id == idx {
			return idx, ids[len(ids)-1], acc, nil
		}
	}
	in := ins[idx]
	ids = append(ids, idx)
	switch in.Operation {
		case "acc":
			return ins.Execute(idx+1, acc+in.Argument, ids)
		case "jmp":
			return ins.Execute(idx+in.Argument, acc, ids)
		case "nop":
			return ins.Execute(idx+1, acc, ids)
		default:
			return 0, 0, 0, fmt.Errorf("No matching command")
	}
}

func check(err error) {
	if err != nil {
		log.Fatalln("Unexpected error:", err)
	}
}
