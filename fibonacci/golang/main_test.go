package main

import (
	"testing"
)

func TestSimpleInputs(t *testing.T) {
	// Tests to run (input, expected).
	tests := []struct {
		input    int32
		expected int32
	}{
		{0, 0},
		{1, 1},
		{4, 3},
		{5, 5},
		{8, 21},
	}

	// Execute each test.
	for _, test := range tests {
		result := Fib(test.input)
		if result != test.expected {
			t.Fatalf("expected: %v received: %v", test.expected, result)
		}
	}
}

func TestLargeInputs(t *testing.T) {
	expected :=
		[]int32{0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811}

	for i := range expected {
		result := Fib(int32(i))
		if expected[i] != result {
			t.Fatalf("expected: %v received: %v", expected[i], result)
		}
	}
}
