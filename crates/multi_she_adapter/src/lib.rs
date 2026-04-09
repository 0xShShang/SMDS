use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use serde::{Deserialize, Serialize};

pub use smds_types::{PbeRequest, PbeResponse, UserEncryptedResponse};

#[cfg(feature = "backend-rust-multi-she")]
pub mod rust_backend;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalPublicParams {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub modulus: BigInt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalSecretKey {
    pub modulus: BigInt,
}

pub trait TwoPartyMultiSheEngine {
    type PublicParams;
    type Server1SecretKey;
    type Server2SecretKey;
    type Ciphertext;
    type PartialCiphertext;
    type Plaintext;

    fn keygen(
        k0: usize,
        k1: usize,
        k2: usize,
    ) -> (Self::PublicParams, Self::Server1SecretKey, Self::Server2SecretKey);

    fn encrypt(
        pp: &Self::PublicParams,
        sk1: &Self::Server1SecretKey,
        sk2: &Self::Server2SecretKey,
        msg: &Self::Plaintext,
    ) -> Self::Ciphertext;

    fn decrypt(
        pp: &Self::PublicParams,
        sk1: &Self::Server1SecretKey,
        sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::Plaintext;

    fn partial_decrypt_by_server2(
        sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::PartialCiphertext;

    fn final_decrypt_by_server1(
        sk1: &Self::Server1SecretKey,
        pct: &Self::PartialCiphertext,
    ) -> Self::Plaintext;

    fn add_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext;
    fn add_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext;

    fn add_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext;
    fn add_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext;

    fn mul_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext;
    fn mul_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext;

    fn mul_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext;
    fn mul_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext;

    fn generate_from_zero_encryptions(
        msg: &Self::Plaintext,
        e0_1: &Self::Ciphertext,
        e0_2: &Self::Ciphertext,
        r1: &Self::Plaintext,
        r2: &Self::Plaintext,
    ) -> Self::Ciphertext;
}

fn default_modulus() -> BigInt {
    (BigInt::one() << 127_u32) - BigInt::one()
}

fn normalize_mod(value: &BigInt, modulus: &BigInt) -> BigInt {
    value.mod_floor(modulus)
}

fn as_bigint(value: &BigInt) -> BigInt {
    value.clone()
}

pub struct LocalMultiSheBackend;

impl LocalMultiSheBackend {
    fn modulus_from_params(pp: &LocalPublicParams) -> &BigInt {
        &pp.modulus
    }
}

#[cfg(feature = "backend-rust-multi-she")]
pub use rust_backend::RustMultiSheBackend;

impl TwoPartyMultiSheEngine for LocalMultiSheBackend {
    type PublicParams = LocalPublicParams;
    type Server1SecretKey = LocalSecretKey;
    type Server2SecretKey = LocalSecretKey;
    type Ciphertext = BigInt;
    type PartialCiphertext = BigInt;
    type Plaintext = BigInt;

    fn keygen(
        k0: usize,
        k1: usize,
        k2: usize,
    ) -> (Self::PublicParams, Self::Server1SecretKey, Self::Server2SecretKey) {
        let modulus = default_modulus();
        let pp = LocalPublicParams {
            k0,
            k1,
            k2,
            modulus: modulus.clone(),
        };
        let sk = LocalSecretKey { modulus };
        (pp, sk.clone(), sk)
    }

    fn encrypt(
        pp: &Self::PublicParams,
        _sk1: &Self::Server1SecretKey,
        _sk2: &Self::Server2SecretKey,
        msg: &Self::Plaintext,
    ) -> Self::Ciphertext {
        normalize_mod(msg, Self::modulus_from_params(pp))
    }

    fn decrypt(
        pp: &Self::PublicParams,
        _sk1: &Self::Server1SecretKey,
        _sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::Plaintext {
        normalize_mod(ct, Self::modulus_from_params(pp))
    }

    fn partial_decrypt_by_server2(
        _sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::PartialCiphertext {
        as_bigint(ct)
    }

    fn final_decrypt_by_server1(
        _sk1: &Self::Server1SecretKey,
        pct: &Self::PartialCiphertext,
    ) -> Self::Plaintext {
        as_bigint(pct)
    }

    fn add_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        a + b
    }

    fn add_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext {
        a + b
    }

    fn add_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext {
        a + b
    }

    fn add_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext {
        a + b
    }

    fn mul_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        a * b
    }

    fn mul_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext {
        a * b
    }

    fn mul_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext {
        a * b
    }

    fn mul_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext {
        a * b
    }

    fn generate_from_zero_encryptions(
        msg: &Self::Plaintext,
        e0_1: &Self::Ciphertext,
        e0_2: &Self::Ciphertext,
        r1: &Self::Plaintext,
        r2: &Self::Plaintext,
    ) -> Self::Ciphertext {
        msg + e0_1 + e0_2 + r1 + r2
    }
}

pub fn normalize_ciphertext(pp: &LocalPublicParams, ct: &BigInt) -> BigInt {
    normalize_mod(ct, &pp.modulus)
}

pub fn normalize_partial_ciphertext(
    pp: &LocalPublicParams,
    ct: &BigInt,
) -> BigInt {
    normalize_mod(ct, &pp.modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen_is_stable() {
        let (pp, sk1, sk2) = LocalMultiSheBackend::keygen(2_048, 40, 160);
        assert_eq!(pp.k0, 2_048);
        assert_eq!(pp.k1, 40);
        assert_eq!(pp.k2, 160);
        assert_eq!(sk1.modulus, pp.modulus);
        assert_eq!(sk2.modulus, pp.modulus);
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let (pp, sk1, sk2) = LocalMultiSheBackend::keygen(2_048, 40, 160);
        let msg = BigInt::from(3_600_u64);
        let ct = LocalMultiSheBackend::encrypt(&pp, &sk1, &sk2, &msg);
        let recovered = LocalMultiSheBackend::decrypt(&pp, &sk1, &sk2, &ct);
        assert_eq!(recovered, msg);
    }

    #[test]
    fn homomorphic_ops_are_exact_in_the_reference_backend() {
        let a = BigInt::from(7_u64);
        let b = BigInt::from(11_u64);
        assert_eq!(LocalMultiSheBackend::add_ct_ct(&a, &b), BigInt::from(18_u64));
        assert_eq!(LocalMultiSheBackend::mul_ct_ct(&a, &b), BigInt::from(77_u64));
    }
}
