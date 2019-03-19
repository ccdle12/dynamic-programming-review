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
