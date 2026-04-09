use std::env;

use smds_bench::run_bangalore_benchmark;
use smds_core::reference_from_price_csv;
use smds_types::ScenarioKind;

fn usage() {
    eprintln!("usage:");
    eprintln!("  smds_cli demo <csv-path>");
    eprintln!("  smds_cli reference <csv-path>");
    eprintln!("  smds_cli benchmark <csv-path> [scenario]");
    eprintln!("  scenarios: baseline | overlap | disjoint | duplicate | single");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "demo".to_string());
    let csv_path = args
        .next()
        .unwrap_or_else(|| "../班加罗尔房地产价格数据集.csv".to_string());
    let scenario_arg = args.next().unwrap_or_else(|| "baseline".to_string());

    match mode.as_str() {
        "demo" => {
            let report = run_bangalore_benchmark(&csv_path, 3, 8, 42, 1)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "reference" => {
            let (_datasets, ranks) = reference_from_price_csv(&csv_path, 3, 8, 42)?;
            println!("{}", serde_json::to_string_pretty(&ranks)?);
        }
        "benchmark" => {
            let scenario = match scenario_arg.as_str() {
                "baseline" => ScenarioKind::BangaloreBaseline,
                "overlap" => ScenarioKind::MultiUserOverlap,
                "disjoint" => ScenarioKind::MultiUserDisjoint,
                "duplicate" => ScenarioKind::DuplicateHeavy,
                "single" => ScenarioKind::SingleUser,
                other => {
                    eprintln!("unknown scenario: {other}");
                    usage();
                    std::process::exit(2);
                }
            };
            let report = smds_bench::run_scenario_benchmark(scenario, &csv_path, 3, 8, 42, 3)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        _ => {
            usage();
            std::process::exit(2);
        }
    }

    Ok(())
}
