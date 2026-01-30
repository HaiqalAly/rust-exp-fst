use std::fs::File;
use std::io::{self};
use std::io::{BufRead, BufReader};

use fst::MapBuilder;

pub fn build_fst(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file).lines();

    let mut keys: Vec<String> = vec![];
    for line in reader {
        let line = line?;
        keys.push(line);
    }
    keys.sort();

    let writer = io::BufWriter::new(File::create(output_path)?);
    let mut build = MapBuilder::new(writer)?;

    for key in keys {
        build.insert(key, 0)?;
    }

    build.finish()?;
    Ok(())
}
