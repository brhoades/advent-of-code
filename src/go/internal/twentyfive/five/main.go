package five

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Part 1: count fresh ingredient IDs.
// Approaches:
//   Naive: construct ranges 1 for 1. Each ingredient looks at each range.
//          O(N*M) where N = range count and M = number of ingredients.
//   Compact: combine ranges / merge them if overlapping
//            Still O(N*M)
//   Better DS: binary tree to store lower value. Would let you find
//       a range quickly. If lower values overlap, needs a vec. Not bad? O(N*M)
//       for all overlap. But in practice, likely closer to O(Mlog(N))
//
// There's a DS out there for quick range incusivity checks with many ranges.
// A tree variant... not a radix tree. Not a red black tree either, though I used
// to confuse it with that.
// _Could_ be a b-tree. Doubt it.
//
// --- yeah, b-tree works but doesn't "solve" it. Would let you find the next
// range if you have overlap. O(Mlog(n)) if I can hold it right. Seems... also not
// worth it.
//
// If ranges were exclusive (they aren't) an inversion list is probably
// what I was thinking of.
//
// I'm going naive.

// Part 2: unique fresh ingredient count in ranges
// At a glance, I suspect a naive approach won't work here. Too many numbers to store and track.
//
// Yeah. This is basically asking for Compact above. Intelligently dedupe or merge ranges.
// I don't think this is too bad; moreso tricky. Merge the ranges with overlap. Then sum their
// spans.
//
// Naive? O(N^2) range merge. Walk each range and compare against other range.
// Worst case: no merges, N^2. It's actually a bit tighter than that, it's N + N-1 + N-2 ... N-(N-1).

func Main(part int, path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	ranges, ingredients, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n%+v\n", ranges, ingredients)

	fresh := freshIngredients(ranges, ingredients)
	fmt.Printf("part 1 result: %d fresh ingredients.\n\t%+v\n", len(fresh), fresh)

	total := totalRange(ranges)
	fmt.Printf("part 2 result: %d total ingredients.\n", total)

	return nil
}

func freshIngredients(ranges []Range, ingredients []int) []int {
	var fresh []int

	for _, ing := range ingredients {
		for _, r := range ranges {
			// inclusive
			if ing >= r.lower && ing <= r.upper {
				fresh = append(fresh, ing)
				break
			}
		}
	}

	return fresh
}

// Returns the sum of all unique ingredient ids spanned by all
// ranges. Approach is to merge and dedupe ranges.
func totalRange(ranges []Range) int {
	// Merges the ranges if possible, returning the resulting ranges.
	// Does not modify passed in ranges.
	merge := func(lhs, rhs Range) []Range {
		// to simplify: the lhs range will always have the lower start
		if lhs.lower > rhs.lower {
			lhs, rhs = rhs, lhs
		}

		// cases:
		// 1. rhs extends lhs, no overlap: [0 10] [10 20] => [0 20]
		// 3. no overlap: [1 2] [5 10]
		// 3. lhs overlaps rhs: [0 15] [10 20] => [0 20]
		// 4. equal: [1 5] [1 5] => [1 5]
		// 5. lhs superset of rhs separate boundaries: [1 10] [3 4] => [1 10]
		// 6. lhs superset of rhs shared left edge: [1 10] [1 4] => [1 10]
		// 7. lhs superset of rhs shared right edge: [1 10] [3 10] => [1 10]
		switch {
		case lhs.upper < rhs.lower:
			// no overlap
		case lhs.lower <= rhs.lower && lhs.upper >= rhs.upper:
			// rhs inside lhs
			return []Range{
				lhs,
			}
		case lhs.upper == rhs.lower:
			// extends perfectly or same, no overlap
			return []Range{
				{
					lower: lhs.lower,
					upper: rhs.upper,
				},
			}
		case lhs.upper > rhs.lower && lhs.lower < rhs.upper:
			// lhs extends into rhs
			return []Range{
				{
					lower: lhs.lower,
					upper: rhs.upper,
				},
			}
		}
		return []Range{lhs, rhs}
	}

	for i, lhs := range ranges {
		for j, rhs := range ranges {
			if i == j {
				continue
			}

			merged := merge(lhs, rhs)
			if len(merged) == 2 {
				continue
			}

			// call ourselves agan, returning the result
			ranges = append(ranges[:i], ranges[i+1:]...)
			if j > i {
				ranges = append(ranges[:j-1], ranges[j:]...)
			} else {
				ranges = append(ranges[:j], ranges[j+1:]...)
			}
			ranges = append(ranges, merged...)

			return totalRange(ranges)
		}
	}

	fmt.Printf("resulting ranges: %+v\n", ranges)
	sum := 0
	for _, rng := range ranges {
		sum += rng.upper - rng.lower + 1
	}
	return sum
}

type Range struct {
	lower int
	upper int
}

// Parses a file which contains N ranges, an empty line, then M numbers.
// Returns the ranges followed by ingredients or an error.
func parse(path string) ([]Range, []int, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to read %s: %w", path, err)
	}

	lines := slices.Collect(strings.Lines(string(bytes)))
	var ranges []Range
	var ingredients []int
	readingIngredients := false

	for i, l := range lines {
		l = strings.Trim(l, "\n")
		switch {
		case l == "":
			readingIngredients = true
		case !readingIngredients:
			parts := strings.SplitN(l, "-", 2)
			if len(parts) != 2 {
				return nil, nil, fmt.Errorf("line %d ('%s') did not have 2 parts to range: %+v", i, l, parts)
			}
			lower, err := strconv.Atoi(parts[0])
			if err != nil {
				return nil, nil, fmt.Errorf("line %d ('%s') did not have numerical lower range: %w", i, l, err)
			}
			upper, err := strconv.Atoi(parts[1])
			if err != nil {
				return nil, nil, fmt.Errorf("line %d ('%s') did not have numerical upper range: %w", i, l, err)
			}

			ranges = append(ranges, Range{
				lower: lower,
				upper: upper,
			})
		case readingIngredients:
			ingredient, err := strconv.Atoi(l)
			if err != nil {
				return nil, nil, fmt.Errorf("line %d ('%s') was not a numerical ingredient: %w", i, l, err)
			}
			ingredients = append(ingredients, ingredient)
		}
	}

	return ranges, ingredients, nil
}
