use std::str::FromStr;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataSet {
    pub name: String,
    pub data: Vec<DataPoint>,
}

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    pub input: Vec<f32>,
    pub output: Vec<f32>,
}

pub fn rnap(dna: &str) -> String {
    return dna
        .chars()
        .map(|n| match n {
            'A' => 'A',
            'a' => 'A',
            'T' => 'U',
            't' => 'U',
            'C' => 'C',
            'c' => 'C',
            'G' => 'G',
            'g' => 'G',
            _ => 'N',
        })
        .collect();
}

pub fn normalize_rna(rna: &str) -> Vec<f32> {
    return rna
        .chars()
        .map(|n| match n {
            'A' => -1.0,
            'C' => -0.25,
            'G' => 0.25,
            'U' => 1.0,
            _ => 0.0,
        })
        .collect();
}

pub fn get_csv_data<T>(path: &str, headers: bool) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let mut data: Vec<Vec<T>> = Vec::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(headers)
        .flexible(true)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        let row: Result<Vec<T>, _> = record.iter().map(|s| s.parse::<T>()).collect();

        data.push(row?);
    }

    return Ok(data);
}
