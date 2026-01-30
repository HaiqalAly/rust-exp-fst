use std::io::{self, Write};

use fst::automaton::Levenshtein;
use fst::{IntoStreamer, Map};
use memmap2::Mmap;
mod build_fst;
use build_fst::build_fst;

// Adapted and built upon from the fst crate examples by the Legendary @burntsushi
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_build = std::time::Instant::now();
    build_fst("dict.txt", "dict.fst")?;
    let duration_build = start_build.elapsed();
    println!("Time to build: {:?}", duration_build);

    let data = std::fs::File::open("dict.fst")?;
    let mmap = unsafe { Mmap::map(&data)? };
    let map = Map::new(mmap)?;

    loop {
        print!("Enter a word to search (type #q to exit): ");
        io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        match input.to_lowercase().as_str() {
            "#q" => break,
            "" => {
                println!("Enter a valid word");
            }
            _ => {
                let start_search = std::time::Instant::now();
                let lev = Levenshtein::new(input.to_lowercase().as_str(), 1)?;
                let stream = map.search(lev).into_stream();
                let matches = stream.into_str_keys()?;
                let duration_search = start_search.elapsed();
                println!("Time to search: {:?}", duration_search);
                println!("{:#?}", matches)
            }
        }
    }
    Ok(())
}
