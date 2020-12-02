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

	validSledCount := 0
	validTobogganCount := 0
	for {
		text, err := rdr.ReadString('\n')
		if err == io.EOF {
			break
		}
		check(err)
		pass := NewPassFromText(text)
		if pass.IsValidSled() {
			validSledCount++
		}
		if pass.IsValidTobbggan() {
			validTobogganCount++
		}
	}

	fmt.Printf("Valid Sled Passwords: %d\n", validSledCount)
	fmt.Printf("Valid Toboggan Passwords: %d\n", validTobogganCount)
}

type Pass struct {
	I    int
	J    int
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

func (p *Pass) IsValidSled() bool {
	var matchCount int
	for _, c := range p.Word {
		if c == p.Char {
			matchCount++
		}
	}
	if matchCount >= p.I && matchCount <= p.J {
		return true
	}
	return false
}

func (p *Pass) IsValidTobbggan() bool {
	iv := p.getRuneAt(p.I) == p.Char
	jv := p.getRuneAt(p.J) == p.Char
	if (iv || jv) && (iv != jv) {
		return true
	}
	return false
}

func (p *Pass) getRuneAt(loc int) rune {
	if len(p.Word) < loc || loc <= 0 {
		return 0
	}
	return rune(p.Word[loc - 1])
}
