use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct ComparisonResult {
    pub changed_rows: Vec<ChangedRow>,
    pub added_rows: Vec<AddedOrRemovedRow>,
    pub removed_rows: Vec<AddedOrRemovedRow>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangedRow {
    pub id: String,
    pub changes: HashMap<String, ChangeValue>,
}

#[derive(Serialize, Deserialize)]
pub struct AddedOrRemovedRow {
    pub id: String,
    pub record: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeValue {
    Single(String),
    Pair(Vec<String>), // [old_value, new_value]
}

pub fn compare_csv_contents(
    file1_content: String,
    file2_content: String,
    comparison_header: String,
) -> Result<ComparisonResult, String> {
    // Normalize the comparison header to remove any extra quotes or whitespace
    let normalized_comparison_header = comparison_header.trim_matches('\"').trim().to_string();

    // Initialize CSV readers
    let mut csv1 = ReaderBuilder::new().from_reader(file1_content.as_bytes());
    let mut csv2 = ReaderBuilder::new().from_reader(file2_content.as_bytes());

    // Read headers
    let headers1: Vec<String> = csv1
        .headers()
        .map_err(|e| format!("Error reading headers from file 1: {}", e))?
        .iter()
        .filter(|h| !h.starts_with("Unnamed") && !h.trim().is_empty())
        .map(|h| h.trim_matches('\"').trim().to_string())
        .collect();

    let headers2: Vec<String> = csv2
        .headers()
        .map_err(|e| format!("Error reading headers from file 2: {}", e))?
        .iter()
        .filter(|h| !h.starts_with("Unnamed") && !h.trim().is_empty())
        .map(|h| h.trim_matches('\"').trim().to_string())
        .collect();

    // Check for comparison header
    if !headers1.iter().any(|h| h == &normalized_comparison_header)
        || !headers2.iter().any(|h| h == &normalized_comparison_header)
    {
        return Err(format!(
            "Comparison header '{}' not found in both CSV files",
            normalized_comparison_header
        ));
    }

    // Create header index maps for quick access
    let header_indices1: HashMap<String, usize> = headers1
        .iter()
        .enumerate()
        .map(|(i, h)| (h.to_string(), i))
        .collect();
    let header_indices2: HashMap<String, usize> = headers2
        .iter()
        .enumerate()
        .map(|(i, h)| (h.to_string(), i))
        .collect();

    // Read records into HashMaps keyed by the comparison header value
    let records_map1 = build_records_map(&mut csv1, &header_indices1, &normalized_comparison_header)?;
    let records_map2 = build_records_map(&mut csv2, &header_indices2, &normalized_comparison_header)?;

    let mut changed_rows = Vec::new();
    let mut added_rows = Vec::new();
    let mut removed_rows = Vec::new();

    // Get all unique IDs
    let all_ids: HashSet<_> = records_map1
        .keys()
        .chain(records_map2.keys())
        .cloned()
        .collect();

    // Compare records
    for id in all_ids {
        match (records_map1.get(&id), records_map2.get(&id)) {
            (Some(record1), Some(record2)) => {
                // Record exists in both files, compare fields
                let mut changes = HashMap::new();

                for header in headers1.iter() {
                    let value1 = record1.get(header).map(String::as_str).unwrap_or("");
                    let value2 = record2.get(header).map(String::as_str).unwrap_or("");

                    if value1 != value2 {
                        changes.insert(
                            header.to_string(),
                            ChangeValue::Pair(vec![value1.to_string(), value2.to_string()]),
                        );
                    }
                }

                if !changes.is_empty() {
                    changed_rows.push(ChangedRow {
                        id: id.clone(),
                        changes,
                    });
                }
            }
            (None, Some(record2)) => {
                // Record added in file2
                added_rows.push(AddedOrRemovedRow {
                    id: id.clone(),
                    record: record2.clone(),
                });
            }
            (Some(record1), None) => {
                // Record removed in file2
                removed_rows.push(AddedOrRemovedRow {
                    id: id.clone(),
                    record: record1.clone(),
                });
            }
            (None, None) => {
                // This should not happen
            }
        }
    }

    Ok(ComparisonResult {
        changed_rows,
        added_rows,
        removed_rows,
    })
}

fn build_records_map(
    csv_reader: &mut csv::Reader<&[u8]>,
    header_indices: &HashMap<String, usize>,
    comparison_header: &str,
) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut records_map = HashMap::new();

    for result in csv_reader.records() {
        let record = result.map_err(|e| format!("Error reading record: {}", e))?;
        let mut record_map = HashMap::new();

        for (header, idx) in header_indices {
            let value = record.get(*idx).unwrap_or("").to_string();
            record_map.insert(header.clone(), value);
        }

        let id = record_map
            .get(comparison_header)
            .ok_or("Missing comparison header in a record")?
            .clone();

        records_map.insert(id, record_map);
    }

    Ok(records_map)
}
