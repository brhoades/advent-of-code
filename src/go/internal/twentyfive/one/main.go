package one

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func One(part int, path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	instruc, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n", instruc)

	fmt.Println("\n\nRunning simulation")
	res := simulate(instruc)
	fmt.Printf("Dial finished on zero %d times\nDial pointed at zero %d times\n", res.endedZero, res.hitZero)

	return nil
}

type simulationResults struct {
	endedZero int // if after an instruction the dial was was at zero
	hitZero   int // if at any point it was on zero, incremented
}

// Simulates following instructions on a dial. The number of times
// the dial passed or ended an instruction on zero is returned.
func simulate(instructions []instruction) (res simulationResults) {
	value := 50

	for step, instruc := range instructions {
		fmt.Printf("Step %d\n", step)
		// nice and stupid, almost how a human would do it
		for i := 0; i < instruc.amount; i++ {
			switch {
			case value == 0 && instruc.is_left:
				fmt.Println("\tvalue wrapped negative")
				value = 99
			case value == 99 && !instruc.is_left:
				fmt.Println("\tvalue wrapped positive")
				value = 0
			case instruc.is_left:
				value -= 1
			case !instruc.is_left:
				value += 1
			default:
				panic("unknown case")
			}

			// part 2: we hit zero
			if value == 0 {
				res.hitZero += 1
			}

			// part 1: ended on zero
			if i == instruc.amount-1 && value == 0 {
				res.endedZero += 1
			}
		}

		fmt.Printf("\tend step. dial is %d. It finished on zero %d times and pointed at zero %d times\n", value, res.endedZero, res.hitZero)
	}

	return
}

type instruction struct {
	is_left bool
	amount  int
}

func parse(path string) ([]instruction, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	lines := slices.Collect(strings.Lines(string(bytes)))
	instructions := make([]instruction, 0, len(lines))

	for _, l := range lines {
		l = strings.Trim(l, "\n\r")

		if len(l) < 2 {
			return nil, fmt.Errorf("line '%s' must be at least two chars long", l)
		}

		amount, err := strconv.Atoi(l[1:])
		if err != nil {
			return nil, fmt.Errorf("line '%s' had number portion ('%s') fail to parse: %w", l, l[1:], err)
		}

		instructions = append(instructions, instruction{
			is_left: l[0] == 'L',
			amount:  amount,
		})
	}

	return instructions, nil
}
