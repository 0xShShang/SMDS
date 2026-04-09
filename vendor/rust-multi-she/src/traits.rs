#![allow(non_snake_case)]

pub trait KeyParamGeneration<KGP> {
    fn key_gen_param(k_0: usize, k_1: usize, k_2: usize) -> KGP {
        Self::KeyGenParam_with_length(k_0, k_1, k_2)
    }

    fn key_gen_param_with_safe_primes(k_0: usize, k_1: usize, k_2: usize) -> KGP {
        Self::KeyGenParam_safe_primes_with_length(k_0, k_1, k_2)
    }

    fn key_gen_param_with_chosen_user_ab(k_0: usize, k_1: usize, k_2: usize) -> (KGP, KGP) {
        Self::KeyGenParamAB(k_0, k_1, k_2)
    }

    fn KeyGenParam_with_length(k_0: usize, k_1: usize, k_2: usize) -> KGP;
    fn KeyGenParam_safe_primes_with_length(k_0: usize, k_1: usize, k_2: usize) -> KGP;
    fn KeyGenParamAB(k_0: usize, k_1: usize, k_2: usize) -> (KGP, KGP);
}

pub trait Encryption<PK, PP, PT, CT> {
    fn encrypt(pk: &PK, pp: &PP, pt: PT) -> CT;
    fn encrypt_with_chosen_user_ab(pk1: &PK, pk2: &PK, pp: &PP, pt: PT) -> CT;
    fn encrypt_with_chosen_user_ab_prerandom(
        pk1: &PK,
        pk2: &PK,
        pp: &PP,
        pt: &CT,
        r_1: &CT,
        rr_1: &CT,
        r_2: &CT,
    ) -> CT;
    fn encrypt_with_chosen_user_ab_I(pk1: &PK, pk2: &PK, pp: &PP, pt: PT) -> CT;
    fn encrypt_with_chosen_user_ab_II(pk1: &PK, pk2: &PK, pp: &PP, ct: &CT) -> CT;
    fn encrypt_in_public_key_setting(PP: &PP, ct1: &CT, ct2: &CT, pt: PT) -> CT;
    fn encrypt_in_public_key_setting_with_prerandom(
        PP: &PP,
        ct1: &CT,
        ct2: &CT,
        pt: &CT,
        r1: &CT,
        r2: &CT,
    ) -> CT;
}

pub trait Decryption<PK, CT, PT> {
    fn decrypt(pk: &PK, ct: CT) -> PT;
    fn decrypt_with_chosen_user_ab(pk1: &PK, pk2: &PK, ct: &CT) -> PT;
    fn decrypt_with_chosen_user_ab_I(pk2: &PK, ct: &CT) -> PT;
    fn decrypt_with_chosen_user_ab_II(pk1: &PK, ct: &CT) -> PT;
}

pub trait Homomorphism<PK, PP, CT, PT> {
    fn s_Add(pp: &PP, ct: &CT, scalar: PT) -> CT;
    fn c_Add(pp: &PP, ct1: &CT, ct2: &CT) -> CT;
    fn s_Mul(pp: &PP, ct: &CT, scalar: PT) -> CT;
    fn c_Mul(pp: &PP, ct1: &CT, ct2: &CT) -> CT;
    fn Multi_sAdd(pk: &PK, pp: &PP, ct: &CT, scalar: PT) -> CT;
    fn Multi_cAdd(pk: &PK, pp: &PP, ct1: &CT, ct2: &CT) -> CT;
    fn Multi_sMul(pk: &PK, pp: &PP, ct: &CT, scalar: PT) -> CT;
    fn Multi_cMul(pk: &PK, pp: &PP, ct1: &CT, ct2: &CT) -> CT;
}

