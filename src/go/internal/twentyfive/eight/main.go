package eight

import (
	"cmp"
	"fmt"
	"maps"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

// Part 1: N shortest M-length graphs of nodes in 3d space.
//
// Seems expensive. We're basically building N minimal spanning trees
// were weight is distance. But they'd rather have N/2 separate trees
// than 1 connected tree. Which implies they're doing an O(n^2) distance
// comparison to build graphs.

func Main(path string) error {
	if len(path) == 0 {
		return fmt.Errorf("invalid path")
	}

	if _, err := os.Stat(path); err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	graph, err := parse(path)
	if err != nil {
		return err
	}

	fmt.Printf("part 1 parse: %+v\n", graph)
	graphs := solve(graph, 1000)
	fmt.Printf("part 1 graph: %+v\n", graphs)

	product := 1
	slices.SortFunc(graphs, func(l, r []coord) int {
		// reverse: descending
		return cmp.Compare(len(r), len(l))
	})
	for i, g := range graphs {
		fmt.Printf("\t%dth largest graph len: %d\n", i+1, len(g))
		if i < 3 {
			product *= len(g)
		}
	}
	fmt.Printf("part 1 product: %d\n", product)

	return nil
}

type coord struct {
	x, y, z int
}

// Given a file with comma-separated 3d coordinates, returns the list
func parse(path string) ([]coord, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read %s: %w", path, err)
	}

	var coords []coord

	for line := range strings.Lines(string(bytes)) {
		line = strings.TrimSpace(line)
		fields := strings.SplitN(line, ",", 3)
		if len(fields) != 3 {
			return nil, fmt.Errorf("line lacked three fields: %q", line)
		}

		coords = append(coords, coord{})
		for i, f := range fields {
			val, err := strconv.Atoi(f)
			if err != nil {
				return nil, fmt.Errorf("field %d failed to parse on line %q: %w", i, line, err)
			}

			switch i {
			case 0:
				coords[len(coords)-1].x = val
			case 1: //
				coords[len(coords)-1].y = val
			case 2:
				coords[len(coords)-1].z = val
			default:
				panic("unreachable, must have 3 fields")
			}
		}
	}

	return coords, nil
}

// Returns the graphs formed by connecting up to maxPair pairs of minimally distant coords. The remainder
// are discarded.
//
// I figured the easiest way to go about this  was to compute the O(N^2) coord distances once
// then sort and make choices. Choosing while sorting has a lot of variable juggling and slice
// mixing which gets yucky.
func solve(coords []coord, maxPairs int) (ret [][]coord) {
	type pair struct {
		lhs, rhs coord
		distance float64
	}
	pairs := make([]pair, 0, len(coords)*len(coords)-len(coords))

	for i, lhs := range coords {
		for _, rhs := range coords[i+1:] {
			pairs = append(pairs, pair{
				lhs:      lhs,
				rhs:      rhs,
				distance: math.Sqrt(math.Pow(float64(lhs.x-rhs.x), 2) + math.Pow(float64(lhs.y-rhs.y), 2) + math.Pow(float64(lhs.z-rhs.z), 2)),
			})
		}
	}

	slices.SortFunc(pairs, func(l, r pair) int {
		return cmp.Compare(l.distance, r.distance)
	})

	type graph struct {
		coords map[coord]struct{}
	}
	var graphs []*graph
	// inclusivity check
	presentInGraph := map[coord]*graph{}

	// now connect
	for _, e := range pairs[:maxPairs] {
		lhs, rhs := e.lhs, e.rhs
		lhsGraph, lhsPresent := presentInGraph[e.lhs]
		rhsGraph, rhsPresent := presentInGraph[e.rhs]

		switch {
		case lhsPresent && !rhsPresent:
			lhsGraph.coords[rhs] = struct{}{}
			presentInGraph[rhs] = lhsGraph
		case rhsPresent && !lhsPresent:
			rhsGraph.coords[lhs] = struct{}{}
			presentInGraph[lhs] = rhsGraph
		case lhsPresent && rhsPresent && lhsGraph == rhsGraph:
		case lhsPresent && rhsPresent:
			// merge rhs into lhs then do NOT continue: new pair!
			for c := range rhsGraph.coords {
				lhsGraph.coords[c] = struct{}{}
				presentInGraph[c] = lhsGraph
			}
			for j := 0; j < len(graphs); j++ {
				if graphs[j] != rhsGraph {
					continue
				}
				if j == len(graphs)-1 {
					graphs = graphs[:j]
				} else {
					graphs = append(graphs[:j], graphs[j+1:]...)
					break
				}
			}
		default:
			g := &graph{
				coords: map[coord]struct{}{
					lhs: {},
					rhs: {},
				},
			}
			graphs = append(graphs, g)
			presentInGraph[lhs] = g
			presentInGraph[rhs] = g
		}
	}

	for i, g := range graphs {
		ret = append(ret, slices.Collect(maps.Keys(g.coords)))
	}

	for _, c := range coords {
		if _, ok := presentInGraph[c]; !ok {
			ret = append(ret, []coord{c})
		}

	}

	return
}
