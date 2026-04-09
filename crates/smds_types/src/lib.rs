use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SmdsParams {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub num_users: usize,
    pub dataset_size: usize,
    pub domain_upper_bound: u64,
}

impl SmdsParams {
    pub fn baseline_bangalore(num_users: usize, dataset_size: usize) -> Self {
        Self {
            k0: 2_048,
            k1: 40,
            k2: 160,
            num_users,
            dataset_size,
            domain_upper_bound: 3_600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserDataset {
    pub values: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalizedDataset {
    pub original_values: Vec<u64>,
    pub dedup_values: Vec<u64>,
    pub original_to_dedup_index: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRecoveredResult {
    pub user_id: usize,
    pub ranks_in_original_order: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserState {
    pub user_id: usize,
    pub sample_seed: u64,
    pub original_values: Vec<u64>,
    pub canonical_values: Vec<u64>,
    pub mask_r_prime: Vec<u64>,
    pub pi1: Permutation,
    pub pi2: Permutation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineBootstrapOutput {
    pub params: SmdsParams,
    pub global_reference: CanonicalizedDataset,
    pub user_states: Vec<UserState>,
    pub root_polynomial: PolynomialCoefficients,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PbeRequest {
    pub user_id: usize,
    pub original_values: Vec<u64>,
    pub masked_values: Vec<u64>,
    pub pi1: Permutation,
    pub pi2: Permutation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PbeResponse {
    pub user_id: usize,
    pub encoded_values: Vec<BigInt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerStageState {
    pub sorted_values: Vec<u64>,
    pub dedup_ranks: Vec<Vec<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserEncryptedResponse {
    pub user_id: usize,
    pub ciphertexts: Vec<BigInt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScenarioKind {
    SingleUser,
    MultiUserDisjoint,
    MultiUserOverlap,
    DuplicateHeavy,
    BangaloreBaseline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkConfig {
    pub scenario: ScenarioKind,
    pub seed: u64,
    pub repetitions: usize,
    pub num_users: usize,
    pub dataset_size: usize,
    pub csv_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StageDurationSummary {
    pub stage: String,
    pub millis: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkReport {
    pub config: BenchmarkConfig,
    pub dataset_summary: DatasetSummary,
    pub reference_ranks: Vec<Vec<u64>>,
    pub protocol_ranks: Vec<Vec<u64>>,
    pub stage_durations: Vec<StageDurationSummary>,
    pub correctness: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DatasetSummary {
    pub total_values: usize,
    pub per_user_lengths: Vec<usize>,
    pub unique_values: usize,
    pub min_value: u64,
    pub max_value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Permutation {
    pub forward: Vec<usize>,
    pub inverse: Vec<usize>,
}

impl Permutation {
    pub fn identity(size: usize) -> Self {
        let forward: Vec<usize> = (0..size).collect();
        let inverse = forward.clone();
        Self { forward, inverse }
    }

    pub fn new(forward: Vec<usize>) -> Result<Self, String> {
        let size = forward.len();
        let mut inverse = vec![usize::MAX; size];

        for (i, &value) in forward.iter().enumerate() {
            if value >= size {
                return Err(format!("permutation value {value} out of range for size {size}"));
            }
            if inverse[value] != usize::MAX {
                return Err("duplicate value in permutation".to_string());
            }
            inverse[value] = i;
        }

        if inverse.iter().any(|&idx| idx == usize::MAX) {
            return Err("permutation is not a bijection".to_string());
        }

        Ok(Self { forward, inverse })
    }

    pub fn apply_indices(&self, indices: &[usize]) -> Vec<usize> {
        indices.iter().map(|&idx| self.forward[idx]).collect()
    }

    pub fn invert_indices(&self, indices: &[usize]) -> Vec<usize> {
        indices.iter().map(|&idx| self.inverse[idx]).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolynomialCoefficients {
    pub coeffs: Vec<BigInt>,
}
