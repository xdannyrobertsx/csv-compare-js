use csv::{ReaderBuilder, StringRecord};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct ComparisonResult {
    added_rows: Vec<HashMap<String, String>>,
    removed_rows: Vec<HashMap<String, String>>,
    changed_rows: Vec<ChangedRow>,
    column_differences: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangedRow {
    row_index: usize,
    changes: HashMap<String, (String, String)>,
}

pub fn compare_csv_contents(
    file1_content: String,
    file2_content: String,
) -> Result<ComparisonResult, String> {
    let mut csv1 = ReaderBuilder::new().from_reader(file1_content.as_bytes());
    let mut csv2 = ReaderBuilder::new().from_reader(file2_content.as_bytes());

    let headers1: HashSet<String> = csv1
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|h| h.to_string())
        .collect();
    let headers2: HashSet<String> = csv2
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let column_differences: Vec<String> =
        headers1.symmetric_difference(&headers2).cloned().collect();

    let (added_rows, removed_rows, changed_rows) = compare_records(&mut csv1, &mut csv2)?;

    Ok(ComparisonResult {
        added_rows,
        removed_rows,
        changed_rows,
        column_differences,
    })
}

fn compare_records(
    csv1: &mut csv::Reader<&[u8]>,
    csv2: &mut csv::Reader<&[u8]>,
) -> Result<
    (
        Vec<HashMap<String, String>>,
        Vec<HashMap<String, String>>,
        Vec<ChangedRow>,
    ),
    String,
> {
    let headers1 = csv1.headers().map_err(|e| e.to_string())?.clone();
    let headers2 = csv2.headers().map_err(|e| e.to_string())?.clone();

    let records1: Vec<StringRecord> = csv1.records().filter_map(Result::ok).collect();
    let records2: Vec<StringRecord> = csv2.records().filter_map(Result::ok).collect();

    let (added_rows, removed_rows) = rayon::join(
        || {
            records2
                .par_iter()
                .skip(records1.len())
                .map(|r| record_to_hashmap(r, &headers2))
                .collect()
        },
        || {
            records1
                .par_iter()
                .skip(records2.len())
                .map(|r| record_to_hashmap(r, &headers1))
                .collect()
        },
    );

    let changed_rows: Vec<ChangedRow> = records1
        .par_iter()
        .enumerate()
        .filter_map(|(i, r1)| {
            records2.get(i).and_then(|r2| {
                let changes = compare_records_row(r1, r2, &headers1, &headers2);
                if !changes.is_empty() {
                    Some(ChangedRow {
                        row_index: i,
                        changes,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    Ok((added_rows, removed_rows, changed_rows))
}

fn record_to_hashmap(record: &StringRecord, headers: &StringRecord) -> HashMap<String, String> {
    headers
        .iter()
        .zip(record.iter())
        .map(|(h, v)| (h.to_string(), v.to_string()))
        .collect()
}

fn compare_records_row(
    r1: &StringRecord,
    r2: &StringRecord,
    headers1: &StringRecord,
    headers2: &StringRecord,
) -> HashMap<String, (String, String)> {
    headers1
        .iter()
        .zip(r1.iter())
        .zip(headers2.iter().zip(r2.iter()))
        .filter_map(|((h1, v1), (h2, v2))| {
            if h1 == h2 && v1 != v2 {
                Some((h1.to_string(), (v1.to_string(), v2.to_string())))
            } else {
                None
            }
        })
        .collect()
}
