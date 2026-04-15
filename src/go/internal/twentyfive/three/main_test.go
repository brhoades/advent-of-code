package three

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

type TestCase struct {
	name   string
	bank   string
	expect int
	size   int
}

func TestPartTwoFourSize(t *testing.T) {
	size := 4

	for _, c := range []TestCase{
		{
			name:   "IdentityOnes",
			bank:   "1111",
			expect: 1111,
		},
		{
			name:   "IdentityDecreasing",
			bank:   "5432",
			expect: 5432,
		},
		{
			name:   "IdentityIncreasing",
			bank:   "1234",
			expect: 1234,
		},
		{
			name:   "RightSubstrIncreasing",
			bank:   "123456789",
			expect: 6789,
		},
		{
			name:   "LeftSubstrDecreasing",
			bank:   "98765432",
			expect: 9876,
		},
		{
			name:   "MiddleSubstrDecreasing",
			bank:   "100098765432000",
			expect: 9876,
		},
		{
			name:   "RightSubstrDecreasing",
			bank:   "100000000098765",
			expect: 9876,
		},
		{
			name:   "FirstLastSplitTwo",
			bank:   "99000000000099",
			expect: 9999,
		},
		{
			name:   "FirstLastSplitTwoZeroPadded",
			bank:   "995321451231990000",
			expect: 9999,
		},
		{
			name:   "InterspersedWorseCase",
			bank:   "819885912591238419",
			expect: 9999,
		},
	} {
		t.Run(c.name, func(t *testing.T) {
			res := solveN([][]int{conv(c.bank)}, size)
			assert.Equal(t, c.expect, res, "in bank %q", c.bank)
		})
	}
}

func TestPartTwoTwelveSize(t *testing.T) {
	size := 12
	for _, c := range []TestCase{
		{
			name:   "ExampleCase199",
			bank:   "7253255271222245116264224262312334252846434226552823612631167265622765821322372433825651223242182531",
			expect: 888865482531,
		},
	} {
		t.Run(c.name, func(t *testing.T) {
			res := solveN([][]int{conv(c.bank)}, size)
			assert.Equal(t, c.expect, res)
		})
	}
}

// converts an int into digits
func conv(s string) (out []int) {
	out = make([]int, 0, len(s))
	for _, d := range strings.Split(s, "") {
		i, _ := strconv.Atoi(d)
		out = append(out, i)
	}

	return out
}
