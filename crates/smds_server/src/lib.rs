use multi_she_adapter::LocalMultiSheBackend;
use smds_core::{plain_dedup_rank, SmdsEngine};
use smds_types::{ServerStageState, UserDataset};

#[derive(Debug, Clone)]
pub struct ServerCoordinator<B = LocalMultiSheBackend> {
    pub engine: SmdsEngine<B>,
}

impl<B> ServerCoordinator<B> {
    pub fn new(engine: SmdsEngine<B>) -> Self {
        Self { engine }
    }
}

impl<B> ServerCoordinator<B>
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
    pub fn stage(&self, datasets: &[UserDataset]) -> ServerStageState {
        self.engine.server_stage(datasets)
    }

    pub fn reference_ranks(&self, datasets: &[UserDataset]) -> Vec<Vec<u64>> {
        plain_dedup_rank(datasets)
    }
}

