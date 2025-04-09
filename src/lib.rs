use serde_wasm_bindgen;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

/// Computes a SHA-256 hash from the given input string.
///
/// # Arguments
///
/// * `input` - The input string to hash.
///
/// # Returns
///
/// A hexadecimal representation of the SHA-256 hash.
///
/// # Errors
///
/// Returns a JavaScript error if the input string is empty.
#[wasm_bindgen]
pub fn compute_sha256(input: &str) -> Result<String, JsValue> {
    if input.is_empty() {
        return Err(JsValue::from_str("Input string cannot be empty"));
    }

    // Hash computation
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result))
}

/// Efficiently generates primes up to the given limit using the Sieve of Eratosthenes.
///
/// # Arguments
///
/// * `limit` - The upper limit for prime number generation.
///
/// # Returns
///
/// A vector containing all prime numbers up to the specified limit.
///
/// # Errors
///
/// Returns a JavaScript error if the provided limit is less than 2 or excessively large.
#[wasm_bindgen]
pub fn generate_primes(limit: usize) -> Result<JsValue, JsValue> {
    if limit < 2 {
        return Err(JsValue::from_str("Limit must be at least 2"));
    }
    if limit > 1_000_000 {
        return Err(JsValue::from_str("Limit too large; must be <= 1_000_000"));
    }

    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize + 1;

    for num in 2..sqrt_limit {
        if sieve[num] {
            for multiple in (num * num..=limit).step_by(num) {
                sieve[multiple] = false;
            }
        }
    }

    let primes: Vec<usize> = sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect();

    serde_wasm_bindgen::to_value(&primes)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use serde_wasm_bindgen;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_compute_sha256() {
        let input = "test";
        let result = compute_sha256(input).unwrap();
        let expected = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        assert_eq!(result, expected);
    }

    #[wasm_bindgen_test]
    fn test_generate_primes() {
        let limit = 10;
        let expected = vec![2, 3, 5, 7];
        let primes_js = generate_primes(limit).unwrap();
        let primes: Vec<usize> = serde_wasm_bindgen::from_value(primes_js).unwrap();
        assert_eq!(primes, expected);
    }
}
