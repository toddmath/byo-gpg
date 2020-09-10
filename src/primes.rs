use num::bigint::{BigUint, RandBigInt, ToBigUint};
use num::integer::Integer;
use num::traits::identities::{One, Zero};
use rand::thread_rng;
use rayon::iter::repeat;
use rayon::prelude::*;

const TRIALS: u32 = 10;
const BIT_SIZE: u64 = 1024;

pub fn gen_large_prime() -> BigUint {
    let (_, large_prime) = repeat(())
        .map(|_| gen_prime_candidate())
        .find_any(|(is_prime, _)| *is_prime)
        .unwrap();

    large_prime
}

fn gen_prime_candidate() -> (bool, BigUint) {
    let mut rng = thread_rng();
    let num = rng.gen_biguint(BIT_SIZE);

    (is_probable_prime(&num), num)
}

fn is_probable_prime(n: &BigUint) -> bool {
    first_twenty_primes(n) && fermat(n) && miller_rabin(n)
}

fn first_twenty_primes(n: &BigUint) -> bool {
    let primes = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71_u32,
    ];

    let zero = BigUint::new(vec![0]);
    for p in primes.iter() {
        if n % p == zero {
            return false;
        }
    }

    true
}

fn fermat(n: &BigUint) -> bool {
    let mut rng = thread_rng();

    let one = BigUint::new(vec![1]);
    let n_minus_1 = n.clone() - 1_u32;

    let a = rng.gen_biguint_below(&n_minus_1);

    a.modpow(&n_minus_1, &n) == one
}

fn miller_rabin(n: &BigUint) -> bool {
    let mut rng = thread_rng();

    if n.is_even() {
        return false;
    }

    for _i in 0..TRIALS {
        let a = rng.gen_biguint_below(&n);
        if is_miller_rabin_witness(&a, &n) {
            return false;
        }
    }

    true
}

fn is_miller_rabin_witness(a: &BigUint, n: &BigUint) -> bool {
    let n_minus_one = n.clone() - 1_u32;
    let (d, s) = factor_as_multiplication(&n_minus_one);
    let s_minus_one = s - 1_u32;

    let mut x = a.modpow(&d, &n);

    if x == BigUint::one() || x == n_minus_one {
        return false;
    }

    let mut j = BigUint::zero();
    while j < s_minus_one {
        let two = 2_u32.to_biguint().unwrap();
        x = x.modpow(&two, &n);
        if x == n_minus_one {
            return false;
        }

        j += 1_u32;
    }

    return true;
}

fn factor_as_multiplication(n: &BigUint) -> (BigUint, BigUint) {
    let mut d = n.clone();
    let mut s = BigUint::new(vec![0]);
    while d.is_even() {
        s += 1_u32;
        d /= 2_u32;
    }

    (d, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_probable_prime_test() {
        let num = BigUint::new(vec![7919]);
        assert!(is_probable_prime(&num));
    }
}