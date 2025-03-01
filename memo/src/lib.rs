use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs;

use anyhow::Result;

pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
  Ok(Vec::new())
//  if fs::exists(&path)? {
  //    let file = BufReader::new(File::open(&path)?);
  //    file.lines().collect()
  //  } else {
  //    Ok(Vec::new())
  //  }
}

pub fn sync(memos: &Vec<String>, path: impl AsRef<Path>) -> Result<()> {
  Ok(())
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_open_a_file() {
    let memos = open("tests/data/memos.txt").unwrap();
    
    assert_eq!(memos, vec!["foo", "bar"], "wrong data");
  }

  #[test]
  fn should_return_empty_vec_for_missing_file() {
    let memos = open("bogus.txt").unwrap();

    assert!(memos.is_empty(), "vec not empty");
  }
}
