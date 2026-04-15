package four

import (
	"fmt"
	"iter"
	"os"
	"slices"
	"strings"
)

func Main(part int, path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	grid, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("%+v\n", grid)

	cnt := removable(grid)
	fmt.Printf("\npart 1 result (removable count): %d\n", len(cnt))

	cntTwo := simulateRemovalRepeated(grid)
	fmt.Printf("\npart 2 result (repeated removal count): %d\n", cntTwo)

	return nil
}

// Returns which rolls of paper ([x, y]) are removable.
func removable(grid [][]bool) (removable [][]int) {
	for x, row := range grid {
		for y, roll := range row {
			if !roll {
				continue
			}

			adjacentSpots := adjacent(x, y, len(row), len(grid))
			occupied := 0
			for pos := range adjacentSpots {
				if grid[pos[0]][pos[1]] {
					occupied += 1
				}

				if occupied >= 4 {
					break
				}
			}

			if occupied < 4 {
				removable = append(removable, []int{x, y})
			}
		}
	}

	return removable
}

// Calculates removable paper. Simulates removal until no more rolls can be removed.
// Returns the total number of removed rolls after.
func simulateRemovalRepeated(input [][]bool) int {
	// avoid editing input
	grid := make([][]bool, len(input))
	for i, row := range input {
		grid[i] = row[:]
	}

	return simulateInner(grid)
}

func simulateInner(grid [][]bool) (cnt int) {
	for _, pos := range removable(grid) {
		grid[pos[0]][pos[1]] = false
		cnt += 1
	}

	if cnt != 0 {
		cnt += simulateInner(grid)
	}
	return cnt
}

// An iterator over accessible [#, #] coordinates from the provided one in a specific rectangle of given bounds.
func adjacent(x, y int, width, height int) iter.Seq[[]int] {
	return func(yield func([]int) bool) {
		if x != 0 {
			// left
			if !yield([]int{x - 1, y}) {
				return
			}
			// left-up
			if y != 0 {
				if !yield([]int{x - 1, y - 1}) {
					return
				}
			}
			// left-down
			if y+1 < height {
				if !yield([]int{x - 1, y + 1}) {
					return
				}
			}
		}

		// up
		if y != 0 {
			if !yield([]int{x, y - 1}) {
				return
			}
		}

		// down
		if y+1 < height {
			if !yield([]int{x, y + 1}) {
				return
			}
		}

		// right
		if x+1 < width {
			if !yield([]int{x + 1, y}) {
				return
			}

			// right-up
			if y != 0 {
				if !yield([]int{x + 1, y - 1}) {
					return
				}
			}

			// right-down
			if y+1 < height {
				if !yield([]int{x + 1, y + 1}) {
					return
				}
			}
		}
	}
}

// Parses a retangular grid of '@' and '.' where '@' is true.
func parse(path string) ([][]bool, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	lines := slices.Collect(strings.Lines(string(bytes)))
	grid := make([][]bool, len(lines))

	for i, l := range lines {
		l = strings.Trim(l, "\n\r")
		l := strings.Split(l, "")

		if len(l) < 2 {
			return nil, fmt.Errorf("line '%s' must be at least two chars long", l)
		}

		grid[i] = make([]bool, len(l))

		for j, c := range l {
			if c == "@" {
				grid[i][j] = true
			}

		}
	}

	return grid, nil
}
