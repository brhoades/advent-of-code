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

	cnt := count(grid)
	fmt.Printf("\npart 1 result: %d\n", cnt)

	return nil
}

// Returns the number of accessible paper rolls.
func count(grid [][]bool) int {
	cnt := 0

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
				cnt += 1
			}
		}
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
