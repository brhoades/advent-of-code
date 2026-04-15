package three

import (
	"fmt"
	"math"
	"os"
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

	banks, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n\n\n", banks)

	res := solveOne(banks)
	fmt.Printf("Part one solution: %d\n", res)

	res = solveN(banks, 12)
	fmt.Printf("Part two solution: %d\n", res)

	return nil
}

// Solves pretty simply. Keep track of the ones and tens, replacing them
// if it is clearly net better by recalculating the candidate value. O(N).
func solveOne(banks Banks) int {
	sum := 0
	for bidx, bank := range banks {
		var tens, ones, candidate int
		replace := func(nt, no int) {
			tens, ones = nt, no
			candidate = tens*10 + ones
			fmt.Printf("candiate for bank %d is now %d\n", bidx, candidate)
		}
		replace(bank[0], bank[1])

		for _, battery := range bank[2:] {
			switch {
			case candidate < ones*10+battery:
				// queue push and pop, removing tens
				replace(ones, battery)
			case candidate < tens*10+battery:
				// queue pop and push, removing ones
				replace(tens, battery)
			}
		}
		sum += candidate
	}

	return sum
}

// Calculates a number from its individual digits that are stored backwards
func calcRev(digits []int) (num int) {
	for i := len(digits) - 1; i >= 0; i-- {
		digit := digits[i]
		num += digit * int(math.Pow10(i))
	}

	return
}

// Provides the sum of the maximum joltage for num in order batteries given a full Banks set.
//
// Works backwards, only accepting new numbers onto the front if at least the size of the leading one.
// Then it finds the first number to "promote" if it's smaller than the subsequent one. By working backwards,
// the index math is annoying
func solveN(banks Banks, size int) int {
	sum := 0

	for bidx, bank := range banks {
		subseq := largestSubseq(bank, size)
		fmt.Printf("largest subseq for bank %d is %+v\n", bidx, subseq)

		num := 0
		for i := range size {
			num += subseq[i] * int(math.Pow10(size-i-1))
		}
		fmt.Printf("As a sum: %d\n", num)
		sum += num
	}

	return sum
}

func largestSubseq(sequence []int, size int) []int {
	subseq := make([]int, 0, size)

	// the bounds of the current search iteration
	start := 0

	if size == len(sequence) {
		return sequence
	}

	for i := range size {
		max, max_idx := -1, -1
		end := len(sequence) - size + i

		for idx := start; idx < end; idx++ {
			num := sequence[idx]

			if num > max {
				max, max_idx = num, idx
			}
			if max == 9 {
				break // cheat, won't be bigger than that
			}
		}

		subseq = append(subseq, max)
		start = max_idx + 1
	}

	fmt.Printf("subseq done %v\n", subseq)

	return subseq
}

// very efficient for small sequences, worst case is a large sequence of all 1's?
func solveNOld(banks Banks, num int) int {
	sum := 0
	for bidx, bank := range banks {
		var batteries []int
		var candidate int
		replace := func(pbank []int) {
			candidate = calcRev(pbank)
			batteries = pbank
			fmt.Printf("candiate for bank %d is now %d\n", bidx, candidate)
		}

		bank = bank[:]
		slices.Reverse(bank)
		replace(bank[:num])

		last := batteries[len(batteries)-1]
		min := slices.Min(batteries)
		for _, battery := range bank[num:] {
			// Never prepend a smaller number since it'll pop something off the end, making the whole
			// thing less.
			if battery < last {
				continue
			}

			idx := len(batteries) - 1
			for i := len(batteries) - 1; i >= 0; i-- {
				// remove first number where the following one is larger (it gets 'promoted')
				if i > 0 && batteries[i] < batteries[i-1] {
					idx = i
					break
				} else if i == 0 && min == batteries[i] {
					idx = i
					break
				}
			}

			nbank := batteries[:idx]
			if idx != len(batteries)-1 {
				nbank = append(nbank, batteries[idx+1:]...)
			}
			nbank = append(nbank, battery)
			replace(nbank)

			last = batteries[len(batteries)-1]
			min = slices.Min(batteries)
		}
		sum += candidate
	}

	return sum
}

type Banks = [][]int

func parse(path string) (banks Banks, err error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	lines := slices.Collect(strings.Lines(string(bytes)))
	linecnt := len(lines)
	banksize := len(lines[0]) - 1 // newline
	banks = make(Banks, linecnt)
	for i := range banks {
		banks[i] = make([]int, 0, banksize)
	}

	for lineno, line := range lines {
		bank := strings.Split(strings.Trim(line, "\n"), "")
		if len(bank) != banksize {
			return nil, fmt.Errorf("line %d has too many batteries, expected %d got %d", lineno, banksize, len(bank))
		}

		for b, battery := range bank {
			start, err := strconv.Atoi(battery)
			if err != nil {
				return nil, fmt.Errorf("line %d failed to parse battery idx %d as number %q: %w", lineno, b, battery, err)
			}
			banks[lineno] = append(banks[lineno], start)
		}
	}

	return
}
