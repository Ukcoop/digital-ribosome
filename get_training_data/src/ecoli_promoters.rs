use std::fs::File;

use serde_json::to_writer_pretty;

use digital_ribosome::utils::{DataPoint, DataSet, get_csv_data, normalize_rna, rnap};

pub fn get_training_data() -> Result<(), Box<dyn std::error::Error>> {
    let promoter_data =
        get_csv_data::<String>("get_training_data/starting_data/ecoli_promoters.csv", true)?;

    let ecoli_data =
        get_csv_data::<String>("get_training_data/starting_data/ecoli_sequence.fasta", true)?;

    let mut promoters: Vec<String> = Vec::new();

    let mut ecoli_gnome: String = String::new();

    let mut dataset: DataSet = DataSet {
        name: "Ecoli promoters".to_string(),
        data: Vec::new(),
    };

    for promoter in promoter_data {
        if promoter[0] != "None" && promoter[1] == "S" {
            promoters.push(rnap(&promoter[0].clone()));
        }
    }

    let mut mask = vec![0.0; promoters[0].len() - 3];
    mask.append(&mut vec![1.0; promoters[0].len()]);
    mask.append(&mut vec![0.0; promoters[0].len() - 3]);

    for section in ecoli_data {
        ecoli_gnome.push_str(rnap(&section[0]).as_str());
    }

    for promoter in promoters {
        let indices: Vec<usize> = ecoli_gnome
            .match_indices(&promoter)
            .map(|(i, _)| i)
            .collect();

        for index in indices {
            let start = index;
            let end = index + promoter.len();

            let rna_slice = &ecoli_gnome[(start - 78)..(end + 78)];

            dataset.data.push(DataPoint {
                input: normalize_rna(rna_slice),
                output: mask.clone(),
            });
        }
    }

    let file = File::create("training_data/ecoli_promoters.json")?;
    to_writer_pretty(file, &dataset)?;

    println!("Done creating training data for Ecoli promoters");

    return Ok(());
}
