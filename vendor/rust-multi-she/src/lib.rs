#![allow(non_snake_case, non_camel_case_types, unused_imports, unused_variables, dead_code)]

pub mod core;
pub mod keygen;
pub mod serialize;
pub mod traits;

pub use crate::core::*;
pub use crate::keygen::*;
pub use crate::serialize::*;
pub use crate::traits::*;

pub use num_bigint::BigInt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Multi_SHE;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyGenParam {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub p: BigInt,
    pub q: BigInt,
    pub L: BigInt,
    pub N: BigInt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriKey {
    pub p: BigInt,
    pub q: BigInt,
    pub L: BigInt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubParam {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub N: BigInt,
}

impl KeyGenParam {
    pub fn key_generation(&self) -> (PriKey, PubParam) {
        (PriKey::from(self), PubParam::from(self))
    }
}

impl<'a> From<&'a KeyGenParam> for PriKey {
    fn from(kgp: &'a KeyGenParam) -> Self {
        Self {
            p: kgp.p.clone(),
            q: kgp.q.clone(),
            L: kgp.L.clone(),
        }
    }
}

impl<'a> From<&'a KeyGenParam> for PubParam {
    fn from(kgp: &'a KeyGenParam) -> Self {
        Self {
            k0: kgp.k0,
            k1: kgp.k1,
            k2: kgp.k2,
            N: kgp.N.clone(),
        }
    }
}
