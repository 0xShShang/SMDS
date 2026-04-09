use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::time::Instant;

use num_bigint::BigInt;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use multi_she_adapter::{LocalMultiSheBackend, TwoPartyMultiSheEngine};
use smds_types::{
    BenchmarkConfig, BenchmarkReport, DatasetSummary, OfflineBootstrapOutput, PbeRequest,
    PolynomialCoefficients, SmdsParams, StageDurationSummary,
    ServerStageState, UserDataset, UserEncryptedResponse, UserRecoveredResult, UserState,
};

pub use smds_crypto_utils::{
    build_root_polynomial, canonicalize_dataset, load_price_column, poly_mul, sample_user_datasets,
    DataError,
};

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn derive_permutation(size: usize, seed: u64, salt: u64) -> smds_types::Permutation {
    let mut indices: Vec<usize> = (0..size).collect();
    indices.sort_by_key(|idx| splitmix64(seed ^ salt ^ (*idx as u64).wrapping_mul(0xD6E8FEB86659FD93)));
    smds_types::Permutation::new(indices).unwrap_or_else(|_| smds_types::Permutation::identity(size))
}

fn derive_mask(values: &[u64], seed: u64, user_id: usize) -> Vec<u64> {
    let mut state = seed ^ ((user_id as u64 + 1) << 1);
    values
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            state = splitmix64(state ^ idx as u64);
            value ^ state
        })
        .collect()
}

fn flatten_datasets(datasets: &[UserDataset]) -> Vec<u64> {
    let mut values = Vec::new();
    for dataset in datasets {
        values.extend_from_slice(&dataset.values);
    }
    values
}

pub fn summarize_datasets(datasets: &[UserDataset]) -> DatasetSummary {
    let flattened = flatten_datasets(datasets);
    let mut unique = BTreeSet::new();
    for &value in &flattened {
        unique.insert(value);
    }
    let per_user_lengths = datasets.iter().map(|dataset| dataset.values.len()).collect();
    let (min_value, max_value) = match (
        flattened.iter().copied().min(),
        flattened.iter().copied().max(),
    ) {
        (Some(min_value), Some(max_value)) => (min_value, max_value),
        _ => (0, 0),
    };

    DatasetSummary {
        total_values: flattened.len(),
        per_user_lengths,
        unique_values: unique.len(),
        min_value,
        max_value,
    }
}

pub fn plain_dedup_rank(datasets: &[UserDataset]) -> Vec<Vec<u64>> {
    let mut unique = BTreeSet::new();
    for dataset in datasets {
        for &value in &dataset.values {
            unique.insert(value);
        }
    }

    let value_to_rank: HashMap<u64, u64> = unique
        .into_iter()
        .enumerate()
        .map(|(idx, value)| (value, (idx + 1) as u64))
        .collect();

    datasets
        .iter()
        .map(|dataset| {
            dataset
                .values
                .iter()
                .map(|value| value_to_rank[value])
                .collect()
        })
        .collect()
}

pub fn reference_from_price_csv(
    path: &str,
    num_users: usize,
    dataset_size: usize,
    seed: u64,
) -> Result<(Vec<UserDataset>, Vec<Vec<u64>>), DataError> {
    let prices = load_price_column(path)?;
    let datasets = sample_user_datasets(&prices, num_users, dataset_size, seed);
    let ranks = plain_dedup_rank(&datasets);
    Ok((datasets, ranks))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SmdsEngine<B = LocalMultiSheBackend> {
    pub params: SmdsParams,
    #[serde(skip)]
    backend: PhantomData<B>,
}

impl<B> SmdsEngine<B> {
    pub fn new(params: SmdsParams) -> Self {
        Self {
            params,
            backend: PhantomData,
        }
    }
}

impl<B> SmdsEngine<B>
where
    B: TwoPartyMultiSheEngine<Ciphertext = BigInt, PartialCiphertext = BigInt, Plaintext = BigInt>,
{
    pub fn build_offline_bootstrap(
        &self,
        datasets: &[UserDataset],
        seed: u64,
    ) -> OfflineBootstrapOutput {
        let flattened = flatten_datasets(datasets);
        let global_reference = canonicalize_dataset(&flattened);
        let root_polynomial = PolynomialCoefficients {
            coeffs: build_root_polynomial(&global_reference.dedup_values),
        };

        let user_states = datasets
            .iter()
            .enumerate()
            .map(|(user_id, dataset)| UserState {
                user_id,
                sample_seed: seed ^ user_id as u64,
                original_values: dataset.values.clone(),
                canonical_values: canonicalize_dataset(&dataset.values).dedup_values,
                mask_r_prime: derive_mask(&dataset.values, seed, user_id),
                pi1: derive_permutation(dataset.values.len(), seed, 0xA1),
                pi2: derive_permutation(dataset.values.len(), seed, 0xB2),
            })
            .collect();

        OfflineBootstrapOutput {
            params: self.params.clone(),
            global_reference,
            user_states,
            root_polynomial,
        }
    }

    pub fn build_pbe_requests(
        &self,
        datasets: &[UserDataset],
        seed: u64,
    ) -> Vec<PbeRequest> {
        datasets
            .iter()
            .enumerate()
            .map(|(user_id, dataset)| {
                let mask_r_prime = derive_mask(&dataset.values, seed, user_id);
                let pi1 = derive_permutation(dataset.values.len(), seed, 0xA1);
                let pi2 = derive_permutation(dataset.values.len(), seed, 0xB2);
                let masked_values = dataset
                    .values
                    .iter()
                    .zip(mask_r_prime.iter())
                    .map(|(value, mask)| value ^ mask)
                    .collect();

                PbeRequest {
                    user_id,
                    original_values: dataset.values.clone(),
                    masked_values,
                    pi1,
                    pi2,
                }
            })
            .collect()
    }

    pub fn server_stage(&self, datasets: &[UserDataset]) -> ServerStageState {
        let mut sorted_values = flatten_datasets(datasets);
        sorted_values.sort_unstable();
        let dedup_ranks = plain_dedup_rank(datasets);
        ServerStageState {
            sorted_values,
            dedup_ranks,
        }
    }

    pub fn encrypt_reference_ranks(
        &self,
        datasets: &[UserDataset],
        _seed: u64,
    ) -> Vec<UserEncryptedResponse> {
        let ranks = plain_dedup_rank(datasets);
        let (pp, sk1, sk2) = B::keygen(self.params.k0, self.params.k1, self.params.k2);

        ranks
            .into_iter()
            .enumerate()
            .map(|(user_id, user_ranks)| {
                let ciphertexts = user_ranks
                    .into_iter()
                    .map(|rank| {
                        let plaintext = BigInt::from(rank);
                        let ct = B::encrypt(&pp, &sk1, &sk2, &plaintext);
                        B::add_ct_plain(&ct, &BigInt::from(0_u32))
                    })
                    .collect();

                UserEncryptedResponse {
                    user_id,
                    ciphertexts,
                }
            })
            .collect()
    }

    pub fn recover_reference_ranks(
        &self,
        encrypted: &[UserEncryptedResponse],
    ) -> Vec<UserRecoveredResult> {
        let (pp, sk1, sk2) = B::keygen(self.params.k0, self.params.k1, self.params.k2);

        encrypted
            .iter()
            .map(|response| {
                let ranks_in_original_order = response
                    .ciphertexts
                    .iter()
                    .map(|ciphertext| {
                        let partial = B::partial_decrypt_by_server2(&sk2, ciphertext);
                        let final_plain = B::final_decrypt_by_server1(&sk1, &partial);
                        let recovered = B::decrypt(&pp, &sk1, &sk2, ciphertext);
                        let value = if recovered == final_plain {
                            recovered
                        } else {
                            final_plain
                        };
                        value.to_u64().unwrap_or_default()
                    })
                    .collect();

                UserRecoveredResult {
                    user_id: response.user_id,
                    ranks_in_original_order,
                }
            })
            .collect()
    }

    pub fn run(&self, datasets: &[UserDataset], seed: u64) -> ProtocolRun {
        let offline = self.build_offline_bootstrap(datasets, seed);
        let pbe_requests = self.build_pbe_requests(datasets, seed);
        let server_stage = self.server_stage(datasets);
        let encrypted_responses = self.encrypt_reference_ranks(datasets, seed);
        let recovered = self.recover_reference_ranks(&encrypted_responses);
        let reference_ranks = plain_dedup_rank(datasets);
        ProtocolRun {
            params: self.params.clone(),
            dataset_summary: summarize_datasets(datasets),
            offline,
            pbe_requests,
            server_stage,
            encrypted_responses,
            recovered,
            reference_ranks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use multi_she_adapter::LocalMultiSheBackend;

    #[test]
    fn reference_ranks_are_global() {
        let datasets = vec![
            UserDataset {
                values: vec![10, 20, 10],
            },
            UserDataset {
                values: vec![20, 30],
            },
        ];
        let ranks = plain_dedup_rank(&datasets);
        assert_eq!(ranks, vec![vec![1, 2, 1], vec![2, 3]]);
    }

    #[test]
    fn engine_pipeline_runs_end_to_end() {
        let engine: SmdsEngine<LocalMultiSheBackend> =
            SmdsEngine::new(SmdsParams::baseline_bangalore(2, 3));
        let datasets = vec![
            UserDataset {
                values: vec![39, 120, 39],
            },
            UserDataset {
                values: vec![120, 300],
            },
        ];
        let run = engine.run(&datasets, 7);
        assert_eq!(run.reference_ranks, vec![vec![1, 2, 1], vec![2, 3]]);
        assert_eq!(run.recovered.len(), 2);
        assert_eq!(run.offline.user_states.len(), 2);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolRun {
    pub params: SmdsParams,
    pub dataset_summary: DatasetSummary,
    pub offline: OfflineBootstrapOutput,
    pub pbe_requests: Vec<PbeRequest>,
    pub server_stage: ServerStageState,
    pub encrypted_responses: Vec<UserEncryptedResponse>,
    pub recovered: Vec<UserRecoveredResult>,
    pub reference_ranks: Vec<Vec<u64>>,
}

pub fn benchmark_report_with_engine<B>(
    engine: &SmdsEngine<B>,
    config: BenchmarkConfig,
    datasets: &[UserDataset],
    seed: u64,
) -> BenchmarkReport
where
    B: TwoPartyMultiSheEngine<Ciphertext = BigInt, PartialCiphertext = BigInt, Plaintext = BigInt>,
{
    let repetitions = config.repetitions.max(1);

    let mut total_offline = 0_u128;
    let mut total_pbe = 0_u128;
    let mut total_server = 0_u128;
    let mut total_encryption = 0_u128;
    let mut total_recovery = 0_u128;
    let mut last_run: Option<ProtocolRun> = None;

    for _ in 0..repetitions {
        let started = Instant::now();
        let offline = engine.build_offline_bootstrap(datasets, seed);
        total_offline += started.elapsed().as_millis();

        let started = Instant::now();
        let pbe_requests = engine.build_pbe_requests(datasets, seed);
        total_pbe += started.elapsed().as_millis();

        let started = Instant::now();
        let server_stage = engine.server_stage(datasets);
        total_server += started.elapsed().as_millis();

        let started = Instant::now();
        let encrypted_responses = engine.encrypt_reference_ranks(datasets, seed);
        total_encryption += started.elapsed().as_millis();

        let started = Instant::now();
        let recovered = engine.recover_reference_ranks(&encrypted_responses);
        total_recovery += started.elapsed().as_millis();

        let reference_ranks = plain_dedup_rank(datasets);
        last_run = Some(ProtocolRun {
            params: engine.params.clone(),
            dataset_summary: summarize_datasets(datasets),
            offline,
            pbe_requests,
            server_stage,
            encrypted_responses,
            recovered,
            reference_ranks,
        });
    }

    let run = last_run.expect("benchmark must run at least once");
    let protocol_ranks: Vec<Vec<u64>> = run
        .recovered
        .iter()
        .map(|result| result.ranks_in_original_order.clone())
        .collect();
    let correctness = run.reference_ranks == protocol_ranks;
    BenchmarkReport {
        config,
        dataset_summary: summarize_datasets(datasets),
        reference_ranks: run.reference_ranks.clone(),
        protocol_ranks,
        stage_durations: vec![
            StageDurationSummary {
                stage: "offline_bootstrap".to_string(),
                millis: total_offline / repetitions as u128,
            },
            StageDurationSummary {
                stage: "pbe_encode".to_string(),
                millis: total_pbe / repetitions as u128,
            },
            StageDurationSummary {
                stage: "server_stage".to_string(),
                millis: total_server / repetitions as u128,
            },
            StageDurationSummary {
                stage: "encrypt_reference_ranks".to_string(),
                millis: total_encryption / repetitions as u128,
            },
            StageDurationSummary {
                stage: "recovery".to_string(),
                millis: total_recovery / repetitions as u128,
            },
        ],
        correctness,
    }
}

pub fn benchmark_report(
    config: BenchmarkConfig,
    datasets: &[UserDataset],
    seed: u64,
) -> BenchmarkReport {
    let engine: SmdsEngine<LocalMultiSheBackend> =
        SmdsEngine::new(SmdsParams::baseline_bangalore(config.num_users, config.dataset_size));
    benchmark_report_with_engine(&engine, config, datasets, seed)
}
