package main

import (
	"testing"
)

func TestNthPrimeNumber(t *testing.T) {
	testCases := []struct {
		input    uint16
		expected uint32
		err      string
	}{
		{1, 2, ""},
		{2, 3, ""},
		{3, 5, ""},
		{4, 7, ""},
		{5, 11, ""},
		{6, 13, ""},
		{7, 17, ""},
		{8, 19, ""},
		{9, 23, ""},
		{10, 29, ""},
		{65535, 821603, ""},
		{0, 0, "Number must be greater than 0"},
	}

	for _, tc := range testCases {
		result := Calculate(tc.input)
		if result.IsErr() {
			if result.Err() == nil || *result.Err() != tc.err {
				t.Errorf("Expected error '%s', got '%v'", tc.err, result.Err())
			}
			continue
		}

		if *result.OK() != tc.expected {
			t.Errorf("For input %d: expected %d but got %d", tc.input, tc.expected, *result.OK())
		}
	}
}
