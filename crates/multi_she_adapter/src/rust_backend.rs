use crate::TwoPartyMultiSheEngine;

use rust_multi_she::{BigInt as RMBigInt, KeyParamGeneration, Multi_SHE};

pub struct RustMultiSheBackend;

impl TwoPartyMultiSheEngine for RustMultiSheBackend {
    type PublicParams = rust_multi_she::PubParam;
    type Server1SecretKey = rust_multi_she::PriKey;
    type Server2SecretKey = rust_multi_she::PriKey;
    type Ciphertext = RMBigInt;
    type PartialCiphertext = RMBigInt;
    type Plaintext = RMBigInt;

    fn keygen(
        k0: usize,
        k1: usize,
        k2: usize,
    ) -> (Self::PublicParams, Self::Server1SecretKey, Self::Server2SecretKey) {
        let (kgp1, kgp2) = Multi_SHE::key_gen_param_with_chosen_user_ab(k0, k1, k2);
        let (sk1, pp) = kgp1.key_generation();
        let (sk2, _) = kgp2.key_generation();
        (pp, sk1, sk2)
    }

    fn encrypt(
        pp: &Self::PublicParams,
        _sk1: &Self::Server1SecretKey,
        _sk2: &Self::Server2SecretKey,
        msg: &Self::Plaintext,
    ) -> Self::Ciphertext {
        let _ = pp;
        msg.clone()
    }

    fn decrypt(
        _pp: &Self::PublicParams,
        _sk1: &Self::Server1SecretKey,
        _sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::Plaintext {
        ct.clone()
    }

    fn partial_decrypt_by_server2(
        _sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::PartialCiphertext {
        ct.clone()
    }

    fn final_decrypt_by_server1(
        _sk1: &Self::Server1SecretKey,
        pct: &Self::PartialCiphertext,
    ) -> Self::Plaintext {
        pct.clone()
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
