package two

import (
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func require[T any](val T, err error) T {
	if err != nil {
		panic(err)
	}

	return val
}

// makes a range from consecutive pairs
func mock(inputs ...int) Range {
	switch len(inputs) {
	case 0:
		return Range{}
	case 2:
	default:
		panic("function accepts only zero or two inputs")
	}

	return Range{
		start: inputs[0],
		end:   inputs[1],
	}
}

func TestPartOne(t *testing.T) {
	type Case struct {
		name   string
		rng    Range
		expect []int
	}

	for _, c := range []Case{
		{
			name:   "EmptyIsZero",
			rng:    mock(),
			expect: nil,
		},
		{
			name:   "InvertedRangeDontCrash",
			rng:    mock(100, 0),
			expect: nil,
		},
		{
			name:   "OneRange",
			rng:    mock(0, 0),
			expect: nil,
		},
		{
			name:   "DigitsBelowTenAreOdd",
			rng:    mock(0, 9),
			expect: nil,
		},
		{
			name:   "TwelveToNineteenHasNothing",
			rng:    mock(12, 19),
			expect: nil,
		},
		{
			name:   "ElevenIsInvalid",
			rng:    mock(11, 11),
			expect: []int{11},
		},
		{
			name:   "ZeroToOneHundredHasTheElevens",
			rng:    mock(0, 100),
			expect: []int{11, 22, 33, 44, 55, 66, 77, 88, 99},
		},
		{
			name:   "BigNum",
			rng:    mock(123123, 123123),
			expect: []int{123123},
		},
	} {
		t.Run(c.name, func(t *testing.T) {
			res := slices.Collect(invalidOne(c.rng))
			assert.Equal(t, c.expect, res)
		})
	}
}
