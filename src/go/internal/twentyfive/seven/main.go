package seven

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

// Part 1: This looks like ray tracing in discrete space. This is almost a simple divide and conquer algorithm,
// except for that beams can merge. I think we can make it work anyway?
//
// Call ourselves on each split, starting at the two edge points around it. Repeat.
// Return back up a slice of coordinates. Dedupe. Count. Profit.
//
// Worked.
//
// Part 2: Oh. I'm glad I took the recursive route. Now I can do this one pretty easily. We're
// doing a total number of unique paths search now, which is basically what my part1 does without the duplicate
// path detection.
//
// Instead? I think we cache coords globally and their result back. Then use that to prevent duplicate path traversal.
// Hell, that's not even global. The result would be stored in that cache at start. We should just generate
// the cache.

func Main(path string) error {
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

	part1Hit := getHitSplitters(grid, grid.start)
	fmt.Printf("\npart 1: %d splitter(s) hit: %+v\n", len(part1Hit), part1Hit)

	part2Paths := getAllPaths(grid, grid.start.x, grid.start.y)
	fmt.Printf("\npart 2: %d timeline(s)\n", part2Paths)

	return nil
}

type coord struct {
	x, y int
}

// stores just where splitters are present in a retangular array of bools
type grid struct {
	splitters     [][]bool
	start         coord
	width, height int
}

// Parses a file which contains whitespace-separated numbers on lines. The final line should be an operation.
func parse(path string) (*grid, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read %s: %w", path, err)
	}

	rows := slices.Collect(strings.Lines(string(bytes)))
	grid := &grid{
		splitters: make([][]bool, len(rows)),
	}
	for i := range rows[0] {
		grid.splitters[i] = make([]bool, len(rows[0]))
	}
	for y, row := range rows {
		for x, col := range row {
			switch col {
			case 'S':
				grid.start.x, grid.start.y = x, y
			case '^':
				grid.splitters[x][y] = true
			}
		}
	}
	grid.height = len(rows)
	grid.width = len(rows[0])

	if grid.start.x == 0 && grid.start.y == 0 {
		return nil, fmt.Errorf("did not find start position")
	}

	return grid, nil
}

// Lazy way to avoid checking splitters twice. Without this, it gets pretty bogged down
// in large maps.
var alreadyVisited = map[coord]struct{}{}

// Returns the coordinates of splitters hit by the beam.
//
// Traces downward until y == height or until a ^ is found. Then recurses.
func getHitSplitters(grid *grid, start coord) []coord {
	x := start.x

	for y := start.y + 1; y < grid.height; y++ {
		spot := coord{
			x: x, y: y,
		}
		fmt.Printf("\tchecking %+v\n", spot)
		if !grid.splitters[x][y] {
			continue
		}
		if _, ok := alreadyVisited[spot]; ok {
			break
		}
		fmt.Printf("\t\thit!\n")
		alreadyVisited[spot] = struct{}{}
		// hit!
		hits := []coord{spot}

		// go to either side
		if x < grid.width-1 {
			hits = append(hits, getHitSplitters(grid, coord{
				x: x + 1,
				y: y,
			})...)
		}
		if x > 0 {
			hits = append(hits, getHitSplitters(grid, coord{
				x: x - 1,
				y: y,
			})...)
		}

		// hack: dedupe if we're the first call
		if start == grid.start {
			return dedupe(hits)
		}
		return hits
	}

	return nil
}

var pathCountCache = map[coord]int{}

// Returns the total number of unique paths possible from a given start.
//
// Traces downward until y == height or until a ^ is found. Then recurses.
func getAllPaths(grid *grid, x, startY int) int {
	for y := startY + 1; y < grid.height; y++ {
		if !grid.splitters[x][y] {
			continue
		}
		c := coord{x: x, y: y}
		if v, ok := pathCountCache[c]; ok {
			return v
		}
		paths := 0

		// go to either side
		if x < grid.width-1 {
			paths += getAllPaths(grid, x+1, y)
		}
		if x > 0 {
			paths += getAllPaths(grid, x-1, y)
		}

		pathCountCache[c] = paths
		return paths
	}

	return 1
}

func dedupe[T comparable](s []T) []T {
	set := map[T]struct{}{}
	for _, v := range s {
		set[v] = struct{}{}
	}

	ret := []T{}
	for v := range set {
		ret = append(ret, v)
	}

	return ret
}
