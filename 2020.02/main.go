package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	dir, err := os.Getwd()
	check(err)
	f, err := os.Open(filepath.Join(dir, "input.txt"))
	check(err)

	rdr := bufio.NewReader(f)

	validCount := 0
	for {
		text, err := rdr.ReadString('\n')
		if err == io.EOF {
			break
		}
		check(err)
		pass := NewPassFromText(text)
		if pass.IsValid() {
			validCount++
		}
	}

	fmt.Printf("Valid Passwords: %d\n", validCount)
}

type Pass struct {
	Min  int
	Max  int
	Char rune
	Word string
}

var rx = regexp.MustCompile(`(\d+)-(\d+) ([a-z]): (\w+)`)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func NewPassFromText(text string) Pass {
	text = strings.TrimSpace(text)
	matches := rx.FindStringSubmatch(text)
	min, err := strconv.Atoi(matches[1])
	check(err)
	max, err := strconv.Atoi(matches[2])
	check(err)
	char := []rune(matches[3])[0]
	word := matches[4]

	return Pass{min, max, char, word}
}

func (p *Pass) IsValid() bool {
	var matchCount int
	for _, c := range p.Word {
		if c == p.Char {
			matchCount++
		}
	}
	if matchCount >= p.Min && matchCount <= p.Max {
		return true
	}
	return false
}
