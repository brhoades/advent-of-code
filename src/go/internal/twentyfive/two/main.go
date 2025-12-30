package two

import (
	"fmt"
	"iter"
	"math"
	"os"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

func Main(part int, path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	ranges, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n\n\n", ranges)

	res := solveOne(ranges)
	fmt.Printf("Part one solution: %d\n", res)

	res = solveTwo(ranges)
	fmt.Printf("Part two solution: %d\n", res)

	res = solveTwoRegex(ranges)
	fmt.Printf("Part two cheeky solution: %d\n", res)

	return nil
}

type Range struct {
	start, end int
}

// solves part one, returning the total sum of invalid product IDs in the range
func solveOne(ranges []Range) (sum int) {
	for _, rng := range ranges {
		for v := range invalidOne(rng) {
			sum += v
		}
	}

	return
}

func solveTwo(ranges []Range) (sum int) {
	for _, rng := range ranges {
		for v := range invalidTwo(rng) {
			sum += v
		}
	}

	return
}

func solveTwoRegex(ranges []Range) (sum int) {
	for _, rng := range ranges {
		for v := range invalidTwoRegex(rng) {
			sum += v
		}
	}

	return
}

// returns all invalid product numbers (numbers that repeat once) in the provided range
func invalidOne(rng Range) iter.Seq[int] {
	return func(yield func(int) bool) {
		for num := rng.start; num <= rng.end; num++ {
			digits := int(math.Floor(math.Log10(float64(num)))) + 1

			// hack cases / easy optimizations
			switch {
			// only numbers below 100 that match are factors of 11
			case num < 100 && num != 0 && num%11 == 0:
				if !yield(num) {
					return
				}
				continue
			case num < 100:
				continue
			case digits%2 == 1:
				// only numbers with even digits can reflect
				continue
			}

			// medium way. Not too bad.
			divisor := int(math.Pow10(digits / 2))
			lhs, rhs := num/divisor, num%divisor
			match := lhs == rhs
			// fmt.Printf("%d in range match ('%d' == '%d'): %t\n", num, lhs, rhs, match)

			// easy way that doesn't make me want to gouge my eyes out
			// s := strconv.Itoa(num)
			// match := s[:midpoint] == s[midpoint:]

			if match && !yield(num) {
				return
			}
		}
	}
}

func nthDigit(num, digit int) int {
	totalDigits := int(math.Log10(float64(num)) + 1)
	divisor := int(math.Pow10(totalDigits - digit))
	return num % divisor / (digit - 1)
}

// returns numbers in the range which repeat N times (n>1)
/*
func invalidTwo(rng Range) iter.Seq[int] {
	return func(yield func(int) bool) {
		for num := rng.start; num <= rng.end; num++ {
			digits := int(math.Floor(math.Log10(float64(num)))) + 1

			repeats := false
			for sliceSz := 1; sliceSz < digits/2; sliceSz++ {
				if digits%sliceSz != 0 {
					// number doesn't partition cleanly
					continue
				}
				if repeats {
					break
				}

				match := true
				for i := 1; i < digits/sliceSz; i++ {
					expect := nthDigit(num, i)
					if !match {
						break
					}

					for j := 1; j < i; j++ {
						if nthDigit(num, digits/sliceSz) != expect {
							match = false
							break
						}
					}
				}
			}

			if repeats && !yield(num) {
				return
			}
		}
	}
}
*/

// walks passed number in increasingly large slices, yielding the number of any slice
// is repeated throughout the whole number cleanly
func invalidTwo(rng Range) iter.Seq[int] {
	return func(yield func(int) bool) {
		for num := rng.start; num <= rng.end; num++ {
			s := []rune(strconv.Itoa(num))
			digits := len(s)
			repeats := false
			if digits < 2 {
				continue
			}

			for sliceSz := 1; sliceSz <= digits/2; sliceSz++ {
				if digits%sliceSz != 0 {
					// number doesn't partition cleanly
					continue
				}

				chunks := slices.Chunk(s, sliceSz)
				var first string
				matches := true
				for c := range chunks {
					switch {
					case first == "":
						first = string(c)
						continue
					case first != string(c):
						matches = false
					default:
						continue
					}
					break
				}

				if matches {
					repeats = true
					break
				}
			}

			if repeats && !yield(num) {
				return
			}
		}
	}
}

// wow, go doesn't support backreferences. How sad.
var repeatsExclusively = regexp.MustCompile(`^(\d{2,})\\1+$`)

// walks passed number in increasingly large slices, yielding the number of any slice
// is repeated throughout the whole number cleanly
func invalidTwoRegex(rng Range) iter.Seq[int] {
	return func(yield func(int) bool) {
		for num := rng.start; num <= rng.end; num++ {
			s := strconv.Itoa(num)
			if repeatsExclusively.MatchString(s) && !yield(num) {
				return
			}
		}
	}
}

func parse(path string) (ranges []Range, err error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	for lineno, rng := range strings.Split(string(bytes), ",") {
		parts := strings.SplitN(rng, "-", 2)
		if len(parts) != 2 {
			return nil, fmt.Errorf("line %d has too many parts: %+v", lineno, parts)
		}

		start, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, fmt.Errorf("line %d failed to parse range open as number in %+v: %w", lineno, parts, err)
		}

		end, err := strconv.Atoi(strings.Trim(parts[1], "\n"))
		if err != nil {
			return nil, fmt.Errorf("line %d failed to parse range close as number in %+v: %w", lineno, parts, err)
		}

		ranges = append(ranges, Range{
			start: start,
			end:   end,
		})
	}

	return
}
