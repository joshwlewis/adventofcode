package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	check(err)

	f, err := os.Open(filepath.Join(wd, "input.txt"))
	check(err)

	oneYesSum, allYesSum, err := SumGroupAnswers(f)
	check(err)

	fmt.Printf("One yes sum: %d\n", oneYesSum)
	fmt.Printf("All yes sum: %d\n", allYesSum)
}

func SumGroupAnswers(f io.Reader) (int, int, error) {
	scnr := bufio.NewScanner(f)
	var group []string
	var oneYesSum, allYesSum int
	for scnr.Scan() {
		person := strings.TrimSpace(scnr.Text())
		if person != "" {
			group = append(group, person)
		} else {
			as := NewAnsSet(strings.Join(group, ""))
			oneYesSum += as.YesCount(1)
			allYesSum += as.YesCount(len(group))
			group = []string{}
		}
	}
	err := scnr.Err()
	return oneYesSum, allYesSum, err
}

type AnsSet map[rune]int

func NewAnsSet(answers string) AnsSet {
	as := AnsSet{}
	for _, answer := range answers {
		as[answer]++
	}
	return as
}

func (as AnsSet) YesCount(min int) (count int) {
	for _, c := range as {
		if c >= min {
			count++
		}
	}
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("Unexpected error: %+v", err)
	}
}
