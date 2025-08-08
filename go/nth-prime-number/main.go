package main

import (
	nthprimenumber "nth-prime-number/gen/betty-blocks/nth-prime-number/nth-prime-number"

	"go.bytecodealliance.org/cm"
)

func init() {
	nthprimenumber.Exports.Calculate = Calculate
}

func Calculate(number uint16) cm.Result[string, uint64, string] {
	if number == 0 {
		return cm.Err[cm.Result[string, uint64, string]]("Number must be greater than 0")
	}

	if number == 1 {
		return cm.OK[cm.Result[string, uint64, string]](uint64(2))
	}

	primes := []uint64{2}
	for i := uint64(3); uint64(len(primes)) < uint64(number); i += 2 {
		if isPrime(i) {
			primes = append(primes, i)
		}
	}
	if len(primes) < int(number) {
		return cm.Err[cm.Result[string, uint64, string]]("Not enough prime numbers found")
	}
	return cm.OK[cm.Result[string, uint64, string]](primes[int(number)-1])
}

func isPrime(n uint64) bool {
	if n <= 1 {
		return false
	}
	for i := uint64(2); i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func main() {}
