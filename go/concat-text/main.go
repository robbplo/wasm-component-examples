package main

import (
	concattext "concat-text/gen/betty-blocks/concat-text/concat-text"
)

func init() {
	concattext.Exports.Concat = Concat
}

func Concat(a string, b string) string {
	return a + b
}

func main() {}
