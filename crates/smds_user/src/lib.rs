use multi_she_adapter::LocalMultiSheBackend;
use smds_core::SmdsEngine;
use smds_types::{PbeRequest, UserDataset, UserState};

#[derive(Debug, Clone)]
pub struct UserClient<B = LocalMultiSheBackend> {
    pub user_id: usize,
    pub engine: SmdsEngine<B>,
}

impl<B> UserClient<B> {
    pub fn new(user_id: usize, engine: SmdsEngine<B>) -> Self {
        Self { user_id, engine }
    }
}

impl<B> UserClient<B>
where
    B: multi_she_adapter::TwoPartyMultiSheEngine<
        PublicParams = multi_she_adapter::LocalPublicParams,
        Server1SecretKey = multi_she_adapter::LocalSecretKey,
        Server2SecretKey = multi_she_adapter::LocalSecretKey,
        Ciphertext = num_bigint::BigInt,
        PartialCiphertext = num_bigint::BigInt,
        Plaintext = num_bigint::BigInt,
    >,
{
    pub fn prepare_state(&self, dataset: &UserDataset, seed: u64) -> UserState {
        self.engine
            .build_offline_bootstrap(std::slice::from_ref(dataset), seed)
            .user_states
            .into_iter()
            .next()
            .unwrap()
    }

    pub fn build_request(&self, dataset: &UserDataset, seed: u64) -> PbeRequest {
        self.engine
            .build_pbe_requests(std::slice::from_ref(dataset), seed)
            .into_iter()
            .next()
            .unwrap()
    }
}

