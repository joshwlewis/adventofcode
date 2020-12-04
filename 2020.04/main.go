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

	validIdCount, err := CountValidIds(f)
	check(err)

	fmt.Printf("Found %d valid ids.", validIdCount)
}

func check(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

type ID map[string]string

func CountValidIds(r io.Reader) (int, error) {
	scnr := bufio.NewScanner(r)
	var count int
	var entry string
	for scnr.Scan() {
		text := scnr.Text()
		entry = fmt.Sprintf("%s\n%s", entry, text)
		if len(strings.TrimSpace(text)) == 0 {
			id, err := NewIdFromEntry(entry)
			if err != nil {
				return 0, err
			}
			if id.IsValid() {
				count++
				fmt.Printf("*Valid*: %+v\n", id)
			} else {
				fmt.Printf("invalid: %+v\n", id)
			}

			entry = ""
		}
	}
	return count, nil
}

func NewIdFromEntry(entry string) (ID, error) {
	pairs := strings.Fields(entry)
	id := make(map[string]string)
	for _, pair := range pairs {
		sides := strings.Split(pair, ":")
		if len(sides) != 2 {
			return nil, fmt.Errorf("unexpected entry: %s\n", pair)
		}
		id[sides[0]] = sides[1]
	}
	return id, nil
}

var requiredKeys = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
// var allowedKeys = append(requiredKeys, "cid")

func (id ID) IsValid() bool {
	for _, key := range requiredKeys {
		if id[key] == "" {
			return false
		}
	}
	return true
}
