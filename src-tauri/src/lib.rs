use csv::{ReaderBuilder, StringRecord};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ComparisonResult {
    pub changed_rows: Vec<ChangedRow>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangedRow {
    pub id: String,
    pub changes: HashMap<String, ChangeValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeValue {
    Single(String),
    Pair(Vec<String>),
}

pub fn compare_csv_contents(
    file1_content: String,
    file2_content: String,
    comparison_header: String,
) -> Result<ComparisonResult, String> {
    let mut csv1 = ReaderBuilder::new().from_reader(file1_content.as_bytes());
    let mut csv2 = ReaderBuilder::new().from_reader(file2_content.as_bytes());

    let headers1: Vec<String> = csv1
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|h| h.to_string())
        .collect();
    let headers2: Vec<String> = csv2
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|h| h.to_string())
        .collect();

    if !headers1.contains(&comparison_header) || !headers2.contains(&comparison_header) {
        return Err(format!("Comparison header '{}' not found in both CSV files", comparison_header));
    }

    let records1: Vec<StringRecord> = csv1.records().filter_map(Result::ok).collect();
    let records2: Vec<StringRecord> = csv2.records().filter_map(Result::ok).collect();

    let mut changed_rows = Vec::new();

    for (record1, record2) in records1.iter().zip(records2.iter()) {
        let id1 = record1.get(headers1.iter().position(|h| h == &comparison_header).unwrap()).unwrap();
        let id2 = record2.get(headers2.iter().position(|h| h == &comparison_header).unwrap()).unwrap();

        if id1 != id2 {
            continue; // Skip rows that don't match on the comparison header
        }

        let mut changes = HashMap::new();
        changes.insert(comparison_header.clone(), ChangeValue::Single(id1.to_string()));

        for (header, (value1, value2)) in headers1.iter().zip(record1.iter().zip(record2.iter())) {
            if header != &comparison_header && value1 != value2 {
                changes.insert(header.to_string(), ChangeValue::Pair(vec![value1.to_string(), value2.to_string()]));
            }
        }

        if changes.len() > 1 { // More than just the ID
            changed_rows.push(ChangedRow {
                id: id1.to_string(),
                changes,
            });
        }
    }

    Ok(ComparisonResult { changed_rows })
}