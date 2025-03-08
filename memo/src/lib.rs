use std::fs::{self, File};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};
use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
  Pending,
  Done,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Memo {
  pub text: String,
  pub status: Status,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Memos {
  path: PathBuf,
  pub inner: Vec<Memo>,
}
 
impl Memos {

  pub fn open(path: impl AsRef<Path>) -> Result<Self> {
    let mut memos = Self {
      path: PathBuf::from(path.as_ref()),
      inner: Vec::new(),
    };

    if fs::exists(&path)? {
      let file = File::open(path.as_ref());
      memos.inner = serde_json::from_reader(BufReader::new(file))?;
    }

    Ok(memos)
  }
  
  pub fn sync(&self) -> Result<()> {
    lef file = File::create(&self.path)?;
    serde_json::to_writer(BufWriter::new(file), &self.inner)?;
    Ok(())
  }

}


#[cfg(test)]
mod tests {
  use tempfile::tempdir;

  use super::*;

  #[test]
  fn should_return_data_from_given_file() {
    let memos = Memos::open("tests/data/memos.txt").unwrap();
    
    assert_eq!(
      memos.inner,
      vec![
          Memo {
                text: "foo".to_string(),
                status: Status::Pending,
          },
          Memo {
            text: "bar".to_string(),
            status: Status::Pending,
          }
      ],
      "wrong data"
    );
  }

  #[test]
  fn should_return_empty_vec_for_missing_file() {
    let memos = Memos::open("bogus.txt").unwrap();

    assert!(memos.inner.is_empty(), "vec not empty");
  }
  
  #[test]
  fn should_write_vec_to_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("memos.txt");
    let memos = Memos {
      path: path.clone(),
      inner: vec![
        Memo {
          text: "foo".to_string(),
          status: Status::Pending,
        },
        Memo {
          text: "bar".to_string(),
          status: Status::Pending,
        }
      ],
    };
    
    memos.sync().unwrap();

    let memos_2 = Memos::open(&path).unwrap();

    assert_eq!(memos.inner, memos_2.inner, "wrong data");
  }
}
