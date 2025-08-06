use crate::exports::betty_blocks::nth_prime_number::nth_prime_number;

wit_bindgen::generate!({ generate_all });

struct PrimeCalulator;

impl nth_prime_number::Guest for PrimeCalulator {
    fn calculate(n: u16) -> Result<u64, String> {
        if n == 0 {
            return Err("Nth prime number must be greater than 0".to_string());
        }

        if n == 1 {
            return Ok(2);
        }

        let mut primes = Vec::new();
        primes.push(2);
        let mut candidate = 2;

        while primes.len() < n as usize {
            if is_prime(candidate, &primes) {
                primes.push(candidate);
            }
            candidate = candidate
                .checked_add(1)
                .ok_or("Overflow detected".to_string())?;
        }

        primes.pop().ok_or("No prime number found".to_string())
    }
}

fn is_prime(n: u64, primes: &[u64]) -> bool {
    if n <= 1 {
        return false;
    }

    for prime in primes {
        if n % prime == 0 {
            return false;
        }
    }

    true
}

export! {PrimeCalulator}

#[cfg(test)]
use crate::exports::betty_blocks::nth_prime_number::nth_prime_number::Guest;

#[test]
fn test_0th_prime_number_errors() {
    let result = PrimeCalulator::calculate(0);
    assert_eq!(
        result,
        Err("Nth prime number must be greater than 0".to_string())
    );
}

#[test]
fn test_2nd_prime_number() {
    let result = PrimeCalulator::calculate(2);
    assert_eq!(result, Ok(3));
}

#[test]
fn test_10th_prime_number() {
    let result = PrimeCalulator::calculate(10);
    assert_eq!(result, Ok(29));
}

#[test]
fn test_max_u16_prime_number() {
    let result = PrimeCalulator::calculate(u16::MAX);
    assert_eq!(result, Ok(821603));
}
