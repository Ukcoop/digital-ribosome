mod ecoli_promoters;

use ecoli_promoters::get_training_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_training_data()?;

    return Ok(());
}
