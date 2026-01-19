use fst::{IntoStreamer, Set};
use fst::automaton::Levenshtein;

// Adapted from the fst crate examples by @burntsushi
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // A convenient way to create sets in memory.
  let keys = vec!["fa", "fo", "fob", "focus", "foo", "food", "foul"];
  let set = Set::from_iter(keys)?;

  // Build our fuzzy query.
  let lev = Levenshtein::new("foo", 2)?;

  // Apply our fuzzy query to the set we built.
  let stream = set.search(lev).into_stream();

  let keys = stream.into_strs()?;
  print!("{:#?}", keys);

  assert_eq!(keys, vec!["fa", "fo", "fob", "foo", "food", "foul"]);
  Ok(())
}