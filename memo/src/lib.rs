use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use std::fs::File;
use std::fs;


pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
  if fs::exists(&path)? {
    let file = BufReader::new(File::open(&path)?);
    file.lines().collect()
  } else {
    Ok(Vec::new())
  }
}

pub fn sync(memos: &[String], path: impl AsRef<Path>) -> Result<()> {
  fs::write(&path, memos.join("\n"))
}

#[cfg(test)]
mod tests {
  use tempfile::tempdir;

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
  
  #[test]
  fn should_write_vec_to_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("memos.txt");
    let vec = vec!["foo".to_string(), "bar".to_string()];
    
    sync(&vec, &path).unwrap();

    let memos = open(&path).unwrap();

    assert_eq!(memos, vec, "wrong data");
  }
}
