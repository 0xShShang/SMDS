use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};

use crate::{
    traits::{Decryption, Encryption, Homomorphism},
    KeyGenParam, Multi_SHE, PriKey, PubParam,
};

fn mod_reduce(value: BigInt, modulus: &BigInt) -> BigInt {
    value.mod_floor(modulus)
}

fn randish(bits: usize) -> BigInt {
    let bits = bits.max(8).min(31);
    BigInt::one() << bits
}

impl Encryption<PriKey, PubParam, u128, BigInt> for Multi_SHE {
    fn encrypt(pk: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt);
        mod_reduce(pt + &pk.L + &pk.p + &pp.N, &pp.N)
    }

    fn encrypt_with_chosen_user_ab(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt);
        mod_reduce((&pt + &pk1.L + &pk2.L + &pp.N) * (&pk1.p + BigInt::one()), &(&pp.N * &pk2.L))
    }

    fn encrypt_with_chosen_user_ab_prerandom(
        pk1: &PriKey,
        pk2: &PriKey,
        pp: &PubParam,
        pt: &BigInt,
        r_1: &BigInt,
        rr_1: &BigInt,
        r_2: &BigInt,
    ) -> BigInt {
        let first_c = (r_1 * pk1.L.clone() + pt) * (BigInt::one() + rr_1 * pk1.p.clone());
        mod_reduce((BigInt::one() + r_2 * pk2.L.clone()) * first_c, &(&pp.N * &pk2.L))
    }

    fn encrypt_with_chosen_user_ab_I(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, pt: u128) -> BigInt {
        let pt = BigInt::from(pt);
        let r_1 = randish(pp.k2);
        let rr_1 = randish(pp.k0);
        Self::encrypt_with_chosen_user_ab_prerandom(pk1, pk2, pp, &pt, &r_1, &rr_1, &BigInt::one())
    }

    fn encrypt_with_chosen_user_ab_II(pk1: &PriKey, pk2: &PriKey, pp: &PubParam, ct: &BigInt) -> BigInt {
        let r_2 = randish(pp.k2);
        mod_reduce((BigInt::one() + r_2 * &pp.N) * ct, &(&pp.N * &pk2.L))
    }

    fn encrypt_in_public_key_setting(PP: &PubParam, ct1: &BigInt, ct2: &BigInt, pt: u128) -> BigInt {
        let pt = BigInt::from(pt);
        mod_reduce(pt + ct1 + ct2, &PP.N)
    }

    fn encrypt_in_public_key_setting_with_prerandom(
        PP: &PubParam,
        ct1: &BigInt,
        ct2: &BigInt,
        pt: &BigInt,
        r1: &BigInt,
        r2: &BigInt,
    ) -> BigInt {
        mod_reduce(pt + r1 * ct1 + r2 * ct2, &PP.N)
    }
}

impl Decryption<PriKey, BigInt, BigInt> for Multi_SHE {
    fn decrypt(pk: &PriKey, ct: BigInt) -> BigInt {
        mod_reduce(ct, &pk.L)
    }

    fn decrypt_with_chosen_user_ab(pk1: &PriKey, pk2: &PriKey, ct: &BigInt) -> BigInt {
        mod_reduce(mod_reduce(ct.clone(), &pk2.L), &pk1.L)
    }

    fn decrypt_with_chosen_user_ab_I(pk2: &PriKey, ct: &BigInt) -> BigInt {
        mod_reduce(ct.clone(), &pk2.L)
    }

    fn decrypt_with_chosen_user_ab_II(pk1: &PriKey, ct: &BigInt) -> BigInt {
        mod_reduce(mod_reduce(ct.clone(), &pk1.p), &pk1.L)
    }
}

impl Homomorphism<PriKey, PubParam, BigInt, usize> for Multi_SHE {
    fn s_Add(pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        mod_reduce(ct + BigInt::from(scalar), &pp.N)
    }

    fn c_Add(pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        mod_reduce(ct1 + ct2, &pp.N)
    }

    fn s_Mul(pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        mod_reduce(ct * BigInt::from(scalar), &pp.N)
    }

    fn c_Mul(pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        mod_reduce(ct1 * ct2, &pp.N)
    }

    fn Multi_sAdd(pk: &PriKey, pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        mod_reduce(ct + BigInt::from(scalar), &(&pp.N * &pk.L))
    }

    fn Multi_cAdd(pk: &PriKey, pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        mod_reduce(ct1 + ct2, &(&pp.N * &pk.L))
    }

    fn Multi_sMul(pk: &PriKey, pp: &PubParam, ct: &BigInt, scalar: usize) -> BigInt {
        mod_reduce(ct * BigInt::from(scalar), &(&pp.N * &pk.L))
    }

    fn Multi_cMul(pk: &PriKey, pp: &PubParam, ct1: &BigInt, ct2: &BigInt) -> BigInt {
        mod_reduce(ct1 * ct2, &(&pp.N * &pk.L))
    }
}

