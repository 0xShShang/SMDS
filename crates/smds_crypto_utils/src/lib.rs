use std::collections::HashMap;
use std::path::Path;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use smds_types::{CanonicalizedDataset, UserDataset};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("failed to open csv: {0}")]
    CsvOpen(#[from] csv::Error),
    #[error("missing price column")]
    MissingPriceColumn,
    #[error("missing value in price column on row {row}")]
    MissingPriceValue { row: usize },
    #[error("invalid price value on row {row}: {value}")]
    InvalidPriceValue { row: usize, value: String },
    #[error("negative price value on row {row}: {value}")]
    NegativePriceValue { row: usize, value: String },
    #[error("failed to read csv record: {0}")]
    CsvRead(#[from] std::io::Error),
}

pub fn load_price_column(path: &str) -> Result<Vec<u64>, DataError> {
    let mut reader = csv::Reader::from_path(Path::new(path))?;
    let headers = reader.headers()?.clone();
    let price_idx = headers
        .iter()
        .position(|field| field == "price")
        .ok_or(DataError::MissingPriceColumn)?;

    let mut prices = Vec::new();
    for (row_idx, record) in reader.records().enumerate() {
        let record = record?;
        let raw = record
            .get(price_idx)
            .ok_or(DataError::MissingPriceValue { row: row_idx + 1 })?
            .trim();

        if raw.is_empty() {
            return Err(DataError::MissingPriceValue { row: row_idx + 1 });
        }

        let price = raw
            .parse::<f64>()
            .map_err(|_| DataError::InvalidPriceValue {
                row: row_idx + 1,
                value: raw.to_string(),
            })?;

        if price.is_sign_negative() {
            return Err(DataError::NegativePriceValue {
                row: row_idx + 1,
                value: raw.to_string(),
            });
        }

        prices.push(price.round() as u64);
    }

    Ok(prices)
}

pub fn sample_user_datasets(
    values: &[u64],
    num_users: usize,
    dataset_size: usize,
    seed: u64,
) -> Vec<UserDataset> {
    if num_users == 0 {
        return Vec::new();
    }

    if values.is_empty() || dataset_size == 0 {
        return (0..num_users)
            .map(|_| UserDataset { values: Vec::new() })
            .collect();
    }

    let mut indices: Vec<usize> = (0..values.len()).collect();
    let mut rng = StdRng::seed_from_u64(seed);
    indices.shuffle(&mut rng);

    let total_needed = num_users.saturating_mul(dataset_size);
    let mut sampled = Vec::with_capacity(num_users);

    for user_id in 0..num_users {
        let start = user_id * dataset_size;
        let mut user_values = Vec::with_capacity(dataset_size);
        for offset in 0..dataset_size {
            let idx = indices[(start + offset) % indices.len()];
            user_values.push(values[idx]);
        }
        sampled.push(UserDataset { values: user_values });
    }

    if total_needed > values.len() {
        // The modulo-based selection intentionally allows wrap-around when the
        // requested sample size exceeds the available pool.
        sampled
    } else {
        sampled
    }
}

pub fn canonicalize_dataset(values: &[u64]) -> CanonicalizedDataset {
    let mut dedup_values = Vec::new();
    let mut seen = HashMap::new();
    let mut original_to_dedup_index = Vec::with_capacity(values.len());

    for &value in values {
        let idx = if let Some(&idx) = seen.get(&value) {
            idx
        } else {
            let idx = dedup_values.len();
            dedup_values.push(value);
            seen.insert(value, idx);
            idx
        };
        original_to_dedup_index.push(idx);
    }

    CanonicalizedDataset {
        original_values: values.to_vec(),
        dedup_values,
        original_to_dedup_index,
    }
}

pub fn poly_mul(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    let mut out = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            out[i + j] += ai * bj;
        }
    }
    out
}

pub fn build_root_polynomial(values: &[u64]) -> Vec<BigInt> {
    let mut coeffs = vec![BigInt::one()];
    for &value in values {
        let factor = vec![BigInt::from(-(value as i64)), BigInt::one()];
        coeffs = poly_mul(&coeffs, &factor);
    }
    coeffs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_price_column_reads_and_rounds_prices() {
        let path = std::env::temp_dir().join("smds_price_test.csv");
        fs::write(
            &path,
            "area_type,availability,location,size,society,total_sqft,bath,balcony,price\n\
             A,Ready,x,1 BHK,s,1000,1,1,39.07\n\
             B,Ready,y,2 BHK,s,1200,2,1,120.0\n",
        )
        .unwrap();

        let prices = load_price_column(path.to_str().unwrap()).unwrap();
        assert_eq!(prices, vec![39, 120]);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn root_polynomial_has_expected_roots() {
        let coeffs = build_root_polynomial(&[2, 3]);
        assert_eq!(coeffs, vec![BigInt::from(6), BigInt::from(-5), BigInt::one()]);
    }
}

