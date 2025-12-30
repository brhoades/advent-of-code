package one

import (
	"fmt"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

// values < 0 are left, values >= 0 are right
func mock(inputs ...int) []instruction {
	instrucs := make([]instruction, 0, len(inputs))
	for _, amount := range inputs {
		instrucs = append(instrucs, instruction{
			is_left: amount < 0,
			amount:  int(math.Abs(float64(amount))),
		})
	}

	return instrucs
}

func TestPartOne(t *testing.T) {
	t.Run("NeverHitsZero", func(t *testing.T) {
		for idx, c := range [][]int{
			{},
			{10},
			{-10},
			{49},
			{-49},
			{5, -5},
			{-5, 5},
			{49, -49},
			{-7, -7, -7, -7, -7, -7, -7},
			{7, 7, 7, 7, 7, 7, 7},
			{5, -5, -10, 10, -49, 49},
		} {
			instr := mock(c...)
			res := simulate(instr)
			assert.Equalf(t, 0, res.endedZero, "on case #%d %+v", idx, instr)
		}
	})

	t.Run("HitZeroOnce", func(t *testing.T) {
		for idx, c := range [][]int{
			{50},
			{-50},
			{25, 25},
			{10, 10, 10, 10, 10},
			{-40, -10},
			{-150},
			{150},
			{100, 20, 10, 5, 5, 2, 2, 2, 2, 1, 1},
		} {
			instr := mock(c...)
			res := simulate(instr)
			assert.Equalf(t, 1, res.endedZero, "on case #%d %+v", idx, instr)
		}
	})
}

func TestPartTwo(t *testing.T) {
	for idx, c := range [][]int{
		{},
		{10},
		{-10},
		{49},
		{-49},
		{5, -5},
		{-5, 5},
		{49, -49},
		{-7, -7, -7, -7, -7, -7, -7},
		{7, 7, 7, 7, 7, 7, 7},
		{5, -5, -10, 10, -49, 49},
	} {
		t.Run(fmt.Sprintf("PassZeroNoneCase%d", idx), func(t *testing.T) {
			instr := mock(c...)
			res := simulate(instr)
			assert.Equalf(t, 0, res.hitZero, "on case #%d %+v", idx, instr)
		})
	}

	for idx, c := range [][]int{
		{51},
		{-51},
		{-25, -10, -10, -6},
		{-50, 50},
		{-50, -50},
		{50, -50},
		{50, 50},
	} {
		t.Run(fmt.Sprintf("PassZeroOnceCase%d", idx), func(t *testing.T) {
			instr := mock(c...)
			res := simulate(instr)
			assert.Equalf(t, 1, res.hitZero, "on case #%d %+v", idx, instr)
		})
	}

	for idx, c := range [][]int{
		{-150, -50},
		{-150, 50},
		{150, -50},
		{150, 50},
	} {
		t.Run(fmt.Sprintf("PassZeroTwiceCase%d", idx), func(t *testing.T) {
			instr := mock(c...)
			res := simulate(instr)
			assert.Equalf(t, 2, res.hitZero, "on case #%d %+v", idx, instr)
		})
	}
}
