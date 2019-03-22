package main

import (
	"testing"
)

func TestBaseCase(t *testing.T) {
	var input int16 = 2
	var expected int16

	result := TripleStep(input)
	if result != expected {
		t.Fatalf("result should have been %v, received: %v", expected, result)
	}
}

func TestRecursive(t *testing.T) {
	// Tests (input, expected).
	tests := []struct {
		input    int16
		expected int16
	}{
		{0, 0},
		{1, 1},
		{2, 0},
		{3, 1},
		{4, 1},
		{5, 2},
		{6, 2},
		{7, 4},
		{8, 5},
		{12, 24},
	}

	// Execute each test.
	for _, test := range tests {
		result := TripleStep(test.input)

		if result != test.expected {
			t.Fatalf("result should have been %v, received: %v",
				test.expected, result)
		}
	}
}

func TestIterative(t *testing.T) {
	// Tests (input, expected).
	tests := []struct {
		input    int16
		expected int16
	}{
		{0, 0},
		{1, 1},
		{2, 0},
		{3, 1},
		{4, 1},
		{5, 2},
		{6, 2},
		{7, 4},
		{8, 5},
		{12, 24},
	}

	// Execute each test.
	for _, test := range tests {
		result := Iterative(test.input)

		if result != test.expected {
			t.Fatalf("result should have been %v, received: %v",
				test.expected, result)
		}
	}
}
