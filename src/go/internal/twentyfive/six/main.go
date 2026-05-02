package six

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Part 1: a parsing problem with a sum at the end. The math here isn't the hard
// part.

// Approaches: I think there's a few bad choices, but the performance woes here won't
// come from string operations. It'll be whatever they require in part 2.
//
// I think lines then contiguous whitespace splitting gives a grid. And it looks like the last
// line is always an operation and all lines are square (or this gets _weird_).

func Main(path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	columns, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n", columns)

	total := solve(columns)
	fmt.Printf("part 1 result: %d\n", total)

	return nil
}

type column struct {
	nums []int
	op   string
}

// returns the solved column's result when applying op to all nums.
func (c *column) total() (total int) {
	if len(c.nums) == 0 {
		return
	}

	total = c.nums[0] // or product is always zero

	for _, num := range c.nums[1:] {
		switch c.op {
		case "+":
			total += num
		case "*":
			total *= num
		}
	}
	return
}

// Parses a file which contains whitespace-separated numbers on lines. The final line should be an operation.
func parse(path string) ([]column, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read %s: %w", path, err)
	}

	rows := slices.Collect(strings.Lines(string(bytes)))
	var ret []column
	for i, row := range rows {
		columns := strings.Fields(row)
		// since we need the first row split to know how many cols we'll have
		if len(ret) == 0 {
			ret = make([]column, len(columns))
		}

		for j, col := range columns {
			if i == len(rows)-1 {
				ret[j].op = col
			} else {
				num, err := strconv.Atoi(col)
				if err != nil {
					return nil, fmt.Errorf("failed to parse '%s' as number on row %d col %d: %w", col, i, j)
				}

				ret[j].nums = append(ret[j].nums, num)
			}
		}
	}

	return ret, nil
}

func solve(cols []column) (sum int) {
	for _, col := range cols {
		sum += col.total()
	}

	return
}
