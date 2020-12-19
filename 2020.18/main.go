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

	sum, err := SumLines(f)
	check(err)

	fmt.Println("Sum", sum)
}

func SumLines(r io.Reader) (sum int, err error) {
	scnr := bufio.NewScanner(r)
	for scnr.Scan() {
		var res Num
		line := strings.TrimSpace(scnr.Text())
		_, res, err = Calculate(line)
		if err != nil {
			return
		}
		sum += int(res)
	}
	err = scnr.Err()
	return
}

func Calculate(input string) (int, Num, error) {
	var operand Operand
	var operator rune
	for i := 0; i < len(input); i++ {
		c := input[i]
		switch c {
		case ' ':
			continue
		case '+', '*':
			operator = rune(c)
		case '(':
			adv, nestedRes, err := Calculate(input[i+1:])
			if err != nil {
				return i + 1, operand.Eval(), err
			}
			if operand == nil {
				operand = nestedRes
			} else {
				switch operator {
				case '+':
					operand = operand.Add(nestedRes)
				case '*':
					operand = operand.Mult(nestedRes)
				}
			}
			i += adv
		case ')':
			return i + 1, operand.Eval(), nil
		default:
			num, err := strconv.Atoi(string(c))
			if err != nil {
				return i + 1, operand.Eval(), err
			}
			switch operator {
			case '+':
				operand = operand.Add(Num(num))
			case '*':
				operand = operand.Mult(Num(num))
			case 0:
				operand = Num(num)
			}
		}
	}
	return len(input), operand.Eval(), nil
}

type Num int
type Operation struct {
	Left Operand
	Right Operand
	Operator rune
}
type Operand interface {
	Add(Num) Operand
	Mult(Num) Operand
	Eval() Num
}
func (n Num) Add(oNum Num) (Operand) {
	return Num(n + oNum)
}
func (n Num) Mult(oNum Num) (Operand) {
	return Operation{n, oNum, '*'}
}
func (n Num) Eval() (Num) {
	return n
}

func (op Operation) Add(oNum Num) (Operand) {
	if rOp, ok := op.Right.(Operation); ok {
		op.Right = rOp.Add(oNum)
	} else if rNum, ok := op.Right.(Num); ok {
		op.Right = rNum + oNum
	}
	return op
}

func (op Operation) Mult(oNum Num) (Operand) {
	op.Right = op.Right.Mult(oNum)
	return op
}

func (op Operation) Eval() (Num) {
	switch op.Operator {
	case '+':
		return op.Left.Eval() + op.Right.Eval()
	case '*':
		return op.Left.Eval() * op.Right.Eval()
	}
	return Num(-1)
}

func (op Operation) Last() (*Operation) {
	if opR, ok := op.Right.(Operation); ok {
		return opR.Last()
	}
	return &op
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected: %v", err)
	}
}
