use num_bigint::BigInt;
use multi_she_adapter::TwoPartyMultiSheEngine;
use smds_core::{
    benchmark_report, benchmark_report_with_engine, load_price_column, sample_user_datasets,
    DataError, SmdsEngine,
};
use smds_types::{BenchmarkConfig, BenchmarkReport, ScenarioKind, UserDataset};

pub fn build_scenario_datasets(
    scenario: ScenarioKind,
    csv_path: &str,
    num_users: usize,
    dataset_size: usize,
    seed: u64,
) -> Result<Vec<UserDataset>, DataError> {
    let prices = load_price_column(csv_path)?;
    let datasets = match scenario {
        ScenarioKind::BangaloreBaseline | ScenarioKind::MultiUserOverlap => {
            sample_user_datasets(&prices, num_users, dataset_size, seed)
        }
        ScenarioKind::SingleUser => {
            let mut users = sample_user_datasets(&prices, 1, dataset_size, seed);
            users.truncate(1);
            users
        }
        ScenarioKind::MultiUserDisjoint => {
            let mut unique_prices = prices.clone();
            unique_prices.sort_unstable();
            unique_prices.dedup();
            if unique_prices.is_empty() {
                Vec::new()
            } else {
                (0..num_users)
                    .map(|user_id| {
                        let start = user_id * dataset_size;
                        let values = (0..dataset_size)
                            .map(|offset| unique_prices[(start + offset) % unique_prices.len()])
                            .collect();
                        UserDataset { values }
                    })
                    .collect()
            }
        }
        ScenarioKind::DuplicateHeavy => {
            let repeated = prices.first().copied().unwrap_or(0);
            (0..num_users)
                .map(|_| UserDataset {
                    values: vec![repeated; dataset_size],
                })
                .collect()
        }
    };
    Ok(datasets)
}

pub fn run_scenario_benchmark(
    scenario: ScenarioKind,
    csv_path: &str,
    num_users: usize,
    dataset_size: usize,
    seed: u64,
    repetitions: usize,
) -> Result<BenchmarkReport, DataError> {
    let datasets = build_scenario_datasets(scenario.clone(), csv_path, num_users, dataset_size, seed)?;
    let config = BenchmarkConfig {
        scenario,
        seed,
        repetitions,
        num_users,
        dataset_size,
        csv_path: csv_path.to_string(),
    };
    Ok(benchmark_report(config, &datasets, seed))
}

pub fn run_scenario_benchmark_with_engine<B>(
    engine: &SmdsEngine<B>,
    scenario: ScenarioKind,
    csv_path: &str,
    num_users: usize,
    dataset_size: usize,
    seed: u64,
    repetitions: usize,
) -> Result<BenchmarkReport, DataError>
where
    B: TwoPartyMultiSheEngine<Ciphertext = BigInt, PartialCiphertext = BigInt, Plaintext = BigInt>,
{
    let datasets = build_scenario_datasets(scenario.clone(), csv_path, num_users, dataset_size, seed)?;
    let config = BenchmarkConfig {
        scenario,
        seed,
        repetitions,
        num_users,
        dataset_size,
        csv_path: csv_path.to_string(),
    };
    Ok(benchmark_report_with_engine(engine, config, &datasets, seed))
}

pub fn run_bangalore_benchmark(
    csv_path: &str,
    num_users: usize,
    dataset_size: usize,
    seed: u64,
    repetitions: usize,
) -> Result<BenchmarkReport, DataError> {
    run_scenario_benchmark(
        ScenarioKind::BangaloreBaseline,
        csv_path,
        num_users,
        dataset_size,
        seed,
        repetitions,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use multi_she_adapter::LocalMultiSheBackend;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn write_csv(contents: &str) -> String {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("smds_bench_{unique}.csv"));
        fs::write(&path, contents).expect("failed to write temp csv");
        path.to_string_lossy().into_owned()
    }

    fn sample_csv() -> String {
        write_csv(
            "area_type,availability,location,size,society,total_sqft,bath,balcony,price\n\
             A,Ready,a,1 BHK,s,1000,1,1,10\n\
             A,Ready,b,1 BHK,s,1001,1,1,20\n\
             A,Ready,c,1 BHK,s,1002,1,1,30\n\
             A,Ready,d,1 BHK,s,1003,1,1,40\n\
             A,Ready,e,1 BHK,s,1004,1,1,50\n\
             A,Ready,f,1 BHK,s,1005,1,1,60\n",
        )
    }

    #[test]
    fn scenarios_generate_expected_shapes() {
        let csv_path = sample_csv();
        let baseline = build_scenario_datasets(ScenarioKind::BangaloreBaseline, &csv_path, 3, 2, 7)
            .expect("baseline datasets");
        let overlap = build_scenario_datasets(ScenarioKind::MultiUserOverlap, &csv_path, 3, 2, 7)
            .expect("overlap datasets");
        let disjoint = build_scenario_datasets(ScenarioKind::MultiUserDisjoint, &csv_path, 3, 2, 7)
            .expect("disjoint datasets");
        let duplicate = build_scenario_datasets(ScenarioKind::DuplicateHeavy, &csv_path, 3, 2, 7)
            .expect("duplicate datasets");
        let single = build_scenario_datasets(ScenarioKind::SingleUser, &csv_path, 3, 2, 7)
            .expect("single datasets");

        assert_eq!(baseline.len(), 3);
        assert_eq!(overlap.len(), 3);
        assert_eq!(disjoint.len(), 3);
        assert_eq!(duplicate.len(), 3);
        assert_eq!(single.len(), 1);
        assert!(duplicate.iter().all(|dataset| dataset.values[0] == dataset.values[1]));
        assert_eq!(disjoint[0].values, vec![10, 20]);
        assert_eq!(disjoint[1].values, vec![30, 40]);
        assert_eq!(disjoint[2].values, vec![50, 60]);
    }

    #[test]
    fn benchmark_report_contains_summary_and_correctness() {
        let csv_path = sample_csv();
        let report = run_scenario_benchmark(ScenarioKind::DuplicateHeavy, &csv_path, 2, 3, 11, 2)
            .expect("benchmark report");
        assert!(report.correctness);
        assert_eq!(report.dataset_summary.total_values, 6);
        assert_eq!(report.dataset_summary.per_user_lengths, vec![3, 3]);
        assert_eq!(report.dataset_summary.unique_values, 1);
        assert_eq!(report.reference_ranks, report.protocol_ranks);
    }

    #[test]
    fn generic_backend_entry_point_matches_local() {
        let csv_path = sample_csv();
        let engine = SmdsEngine::<LocalMultiSheBackend>::new(
            smds_types::SmdsParams::baseline_bangalore(2, 3),
        );
        let report = run_scenario_benchmark_with_engine(
            &engine,
            ScenarioKind::BangaloreBaseline,
            &csv_path,
            2,
            3,
            11,
            2,
        )
        .expect("benchmark report");
        assert!(report.correctness);
    }
}
