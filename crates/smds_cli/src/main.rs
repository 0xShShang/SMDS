use std::env;

use smds_bench::run_scenario_benchmark;
use smds_core::reference_from_price_csv;
use smds_types::ScenarioKind;

#[cfg(feature = "backend-rust-multi-she")]
use multi_she_adapter::RustMultiSheBackend;
#[cfg(feature = "backend-rust-multi-she")]
use smds_bench::run_scenario_benchmark_with_engine;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackendChoice {
    Local,
    Rust,
}

impl BackendChoice {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "local" => Ok(Self::Local),
            "rust" => Ok(Self::Rust),
            other => Err(format!("unknown backend: {other}")),
        }
    }
}

fn usage() {
    eprintln!("usage:");
    eprintln!("  smds_cli demo <csv-path> [backend]");
    eprintln!("  smds_cli reference <csv-path>");
    eprintln!("  smds_cli benchmark <csv-path> [scenario] [backend]");
    eprintln!("  scenarios: baseline | overlap | disjoint | duplicate | single");
    eprintln!("  backends: local | rust");
}

fn parse_scenario(value: &str) -> Result<ScenarioKind, String> {
    match value {
        "baseline" => Ok(ScenarioKind::BangaloreBaseline),
        "overlap" => Ok(ScenarioKind::MultiUserOverlap),
        "disjoint" => Ok(ScenarioKind::MultiUserDisjoint),
        "duplicate" => Ok(ScenarioKind::DuplicateHeavy),
        "single" => Ok(ScenarioKind::SingleUser),
        other => Err(format!("unknown scenario: {other}")),
    }
}

fn run_demo(csv_path: &str, backend: BackendChoice) -> Result<(), Box<dyn std::error::Error>> {
    match backend {
        BackendChoice::Local => {
            let report = run_scenario_benchmark(ScenarioKind::BangaloreBaseline, csv_path, 3, 8, 42, 1)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        BackendChoice::Rust => {
            #[cfg(feature = "backend-rust-multi-she")]
            {
                let engine = smds_core::SmdsEngine::<RustMultiSheBackend>::new(
                    smds_types::SmdsParams::baseline_bangalore(3, 8),
                );
                let report = run_scenario_benchmark_with_engine(
                    &engine,
                    ScenarioKind::BangaloreBaseline,
                    csv_path,
                    3,
                    8,
                    42,
                    1,
                )?;
                println!("{}", serde_json::to_string_pretty(&report)?);
            }
            #[cfg(not(feature = "backend-rust-multi-she"))]
            {
                return Err("rust backend feature is not enabled at compile time".into());
            }
        }
    }

    Ok(())
}

fn run_benchmark(
    csv_path: &str,
    scenario: ScenarioKind,
    backend: BackendChoice,
) -> Result<(), Box<dyn std::error::Error>> {
    match backend {
        BackendChoice::Local => {
            let report = run_scenario_benchmark(scenario, csv_path, 3, 8, 42, 3)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        BackendChoice::Rust => {
            #[cfg(feature = "backend-rust-multi-she")]
            {
                let engine = smds_core::SmdsEngine::<RustMultiSheBackend>::new(
                    smds_types::SmdsParams::baseline_bangalore(3, 8),
                );
                let report = run_scenario_benchmark_with_engine(
                    &engine,
                    scenario,
                    csv_path,
                    3,
                    8,
                    42,
                    3,
                )?;
                println!("{}", serde_json::to_string_pretty(&report)?);
            }
            #[cfg(not(feature = "backend-rust-multi-she"))]
            {
                return Err("rust backend feature is not enabled at compile time".into());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "demo".to_string());
    let csv_path = args
        .next()
        .unwrap_or_else(|| "../班加罗尔房地产价格数据集.csv".to_string());

    match mode.as_str() {
        "demo" => {
            let backend = args
                .next()
                .map(|value| BackendChoice::parse(&value))
                .transpose()
                .map_err(|err| -> Box<dyn std::error::Error> { err.into() })?
                .unwrap_or(BackendChoice::Local);
            run_demo(&csv_path, backend)?;
        }
        "reference" => {
            let (_datasets, ranks) = reference_from_price_csv(&csv_path, 3, 8, 42)?;
            println!("{}", serde_json::to_string_pretty(&ranks)?);
        }
        "benchmark" => {
            let scenario = args
                .next()
                .map(|value| parse_scenario(&value))
                .transpose()
                .map_err(|err| -> Box<dyn std::error::Error> { err.into() })?
                .unwrap_or(ScenarioKind::BangaloreBaseline);
            let backend = args
                .next()
                .map(|value| BackendChoice::parse(&value))
                .transpose()
                .map_err(|err| -> Box<dyn std::error::Error> { err.into() })?
                .unwrap_or(BackendChoice::Local);
            run_benchmark(&csv_path, scenario, backend)?;
        }
        _ => {
            usage();
            std::process::exit(2);
        }
    }

    Ok(())
}
