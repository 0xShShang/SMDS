use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

use crate::{traits::KeyParamGeneration, KeyGenParam, Multi_SHE};

fn bounded_seed(bitsize: usize, salt: usize) -> BigInt {
    let bits = bitsize.max(8).min(31);
    let base = BigInt::one() << bits;
    let tweak = BigInt::from((salt as u64 % 997) + 1);
    base + tweak
}

impl KeyParamGeneration<KeyGenParam> for Multi_SHE {
    fn KeyGenParam_with_length(k_0: usize, k_1: usize, k_2: usize) -> KeyGenParam {
        let p = bounded_seed(k_0, 11);
        let q = bounded_seed(k_0, 19);
        let L = bounded_seed(k_2, 23);
        let N = &p * &q;
        KeyGenParam { k0: k_0, k1: k_1, k2: k_2, p, q, L, N }
    }

    fn KeyGenParam_safe_primes_with_length(k_0: usize, k_1: usize, k_2: usize) -> KeyGenParam {
        Self::KeyGenParam_with_length(k_0, k_1, k_2)
    }

    fn KeyGenParamAB(k_0: usize, k_1: usize, k_2: usize) -> (KeyGenParam, KeyGenParam) {
        let p = bounded_seed(k_0, 11);
        let q = bounded_seed(k_0, 19);
        let l1 = bounded_seed(k_2, 23);
        let l2 = bounded_seed(k_2.saturating_add(k_0), 29);
        let N = &p * &q;
        (
            KeyGenParam {
                k0: k_0,
                k1: k_1,
                k2: k_2,
                p: p.clone(),
                q: q.clone(),
                L: l1,
                N: N.clone(),
            },
            KeyGenParam {
                k0: k_0,
                k1: k_1,
                k2: k_2,
                p: BigInt::zero(),
                q: BigInt::zero(),
                L: l2,
                N,
            },
        )
    }
}

