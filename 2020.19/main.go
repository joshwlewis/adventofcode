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

	sys, err := Parse(f)
	check(err)

	valMsgs := sys.CountMatching(Ref(0))
	fmt.Println("Messages that match Rule 0", valMsgs)
}

type Message string
type Letter rune
type Ref int
type Series [2]Rule
type Option [2]Rule
type Rule interface {
	Messages(*System) map[Message]bool
}
type System struct {
	Messages []Message
	Rules    map[Ref]Rule
	Cache    map[Ref]map[Message]bool
}

func (sys System) CountMatching(r Ref) (sum int) {
	valMsgs := sys.Rules[r].Messages(&sys)
	fmt.Println("valmsgs", len(valMsgs))
	for _, msg := range sys.Messages {
		if val, ok := valMsgs[msg]; val && ok {
			sum++
		}
	}
	return
}

func (l Letter) Messages(sys *System) map[Message]bool {
	msgs := make(map[Message]bool)
	msgs[Message(string(l))] = true
	return msgs
}

func (s Series) Messages(sys *System) map[Message]bool {
	msgs := make(map[Message]bool)
	for msgA := range s[0].Messages(sys) {
		for msgB := range s[1].Messages(sys) {
			msgs[msgA+msgB] = true
		}
	}
	return msgs
}

func (r Ref) Messages(sys *System) map[Message]bool {
	var ms map[Message]bool
	if ms, ok := sys.Cache[r]; ok {
		return ms
	}
	fmt.Println("Looking up R", r)
	os.Chmod("foo", os.FileMode(4))
	switch {
	case r == Ref(8):
		ms = Ref(-2).Messages(sys)
	case r == Ref(11):
		ms = Ref(-1).Messages(sys)
	case r < -6:
		ms = map[Message]bool{Message("x"): true}
	case r < 0 && (r%2 == 0):
		ms = Option{Ref(42), Series{Ref(42), Ref(r - 2)}}.Messages(sys)
	case r < 0:
		ms = Option{Series{Ref(42), Ref(31)}, Series{Series{Ref(42), Ref(r - 2)}, Ref(31)}}.Messages(sys)
	default:
		ms = sys.Rules[r].Messages(sys)
	}
	return ms
}

func (o Option) Messages(sys *System) map[Message]bool {
	msgs := make(map[Message]bool)
	for _, s := range o {
		for m := range s.Messages(sys) {
			msgs[m] = true
		}
	}
	return msgs
}

func Parse(r io.Reader) (sys System, err error) {
	sys = System{Cache: make(map[Ref]map[Message]bool), Rules: make(map[Ref]Rule)}
	scnr := bufio.NewScanner(r)
	var group int
	for scnr.Scan() {
		line := strings.TrimSpace(scnr.Text())
		if line == "" {
			group++
		}
		if group == 1 {
			sys.Messages = append(sys.Messages, Message(line))
			continue
		}
		parts := strings.Split(line, ": ")
		var key int
		key, err = strconv.Atoi(parts[0])
		if err != nil {
			return
		}
		keyRef := Ref(key)

		rightParts := strings.Split(parts[1], " ")
		switch len(rightParts) {
		case 1:
			var num int
			num, err = strconv.Atoi(rightParts[0])
			if err == nil {
				sys.Rules[keyRef] = Ref(num)
			} else {
				err = nil
				sys.Rules[keyRef] = Letter(rightParts[0][1])
			}
		case 2:
			var rightNums []int
			for _, s := range rightParts {
				var num int
				num, err = strconv.Atoi(s)
				if err != nil {
					return
				}
				rightNums = append(rightNums, num)
			}
			sys.Rules[keyRef] = Series{Ref(rightNums[0]), Ref(rightNums[1])}
		case 3:
			var rightNums []int
			for i, s := range rightParts {
				if i == 1 {
					continue
				}
				var num int
				num, err = strconv.Atoi(s)
				if err != nil {
					return
				}
				rightNums = append(rightNums, num)
			}
			sys.Rules[keyRef] = Option{Ref(rightNums[0]), Ref(rightNums[1])}
		case 5:
			var rightNums []int
			for i, s := range rightParts {
				if i == 2 {
					continue
				}
				var num int
				num, err = strconv.Atoi(s)
				if err != nil {
					return
				}
				rightNums = append(rightNums, num)
			}
			sys.Rules[keyRef] = Option{
				Series{Ref(rightNums[0]), Ref(rightNums[1])},
				Series{Ref(rightNums[2]), Ref(rightNums[3])},
			}
		}
	}
	err = scnr.Err()
	return
}

func check(err error) {
	if err != nil {
		log.Fatalf("unexpected: %v", err)
	}
}
