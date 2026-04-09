use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

use crate::{LocalPublicParams, LocalSecretKey, TwoPartyMultiSheEngine};

use rust_multi_she::{
    BigInt as RMBigInt, Decryption, Encryption, Homomorphism, KeyParamGeneration, Multi_SHE,
};

pub struct RustMultiSheBackend;

fn to_u128_plaintext(value: &BigInt) -> u128 {
    value
        .to_u128()
        .unwrap_or_else(|| panic!("plaintext value {value} does not fit into u128"))
}

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
        sk1: &Self::Server1SecretKey,
        sk2: &Self::Server2SecretKey,
        msg: &Self::Plaintext,
    ) -> Self::Ciphertext {
        Multi_SHE::encrypt_with_chosen_user_ab(sk1, sk2, pp, to_u128_plaintext(msg))
    }

    fn decrypt(
        pp: &Self::PublicParams,
        sk1: &Self::Server1SecretKey,
        sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::Plaintext {
        let _ = pp;
        Multi_SHE::decrypt_with_chosen_user_ab(sk1, sk2, ct)
    }

    fn partial_decrypt_by_server2(
        sk2: &Self::Server2SecretKey,
        ct: &Self::Ciphertext,
    ) -> Self::PartialCiphertext {
        Multi_SHE::decrypt_with_chosen_user_ab_I(sk2, ct)
    }

    fn final_decrypt_by_server1(
        sk1: &Self::Server1SecretKey,
        pct: &Self::PartialCiphertext,
    ) -> Self::Plaintext {
        Multi_SHE::decrypt_with_chosen_user_ab_II(sk1, pct)
    }

    fn add_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::c_Add(&pp, a, b)
    }

    fn add_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::c_Add(&pp, a, b)
    }

    fn add_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::s_Add(&pp, a, to_u128_plaintext(b))
    }

    fn add_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::s_Add(&pp, a, to_u128_plaintext(b))
    }

    fn mul_ct_ct(a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::c_Mul(&pp, a, b)
    }

    fn mul_pct_pct(
        a: &Self::PartialCiphertext,
        b: &Self::PartialCiphertext,
    ) -> Self::PartialCiphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::c_Mul(&pp, a, b)
    }

    fn mul_ct_plain(a: &Self::Ciphertext, b: &Self::Plaintext) -> Self::Ciphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::s_Mul(&pp, a, to_u128_plaintext(b))
    }

    fn mul_pct_plain(
        a: &Self::PartialCiphertext,
        b: &Self::Plaintext,
    ) -> Self::PartialCiphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::s_Mul(&pp, a, to_u128_plaintext(b))
    }

    fn generate_from_zero_encryptions(
        msg: &Self::Plaintext,
        e0_1: &Self::Ciphertext,
        e0_2: &Self::Ciphertext,
        r1: &Self::Plaintext,
        r2: &Self::Plaintext,
    ) -> Self::Ciphertext {
        let pp = rust_multi_she::PubParam {
            k0: 0,
            k1: 0,
            k2: 0,
            N: BigInt::zero(),
        };
        Multi_SHE::encrypt_in_public_key_setting_with_prerandom(
            &pp,
            e0_1,
            e0_2,
            msg,
            r1,
            r2,
        )
    }
}
