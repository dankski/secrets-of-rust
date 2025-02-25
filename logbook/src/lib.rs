use anyhow::Result;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;

pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
  if fs::exists(&path)? {
    let text = fs::read_to_string(path)?;
    if text.is_empty() {
      Ok(None)
    } else {
      Ok(Some(text))
    }    
  } else {
   Ok(None)
  }
}

pub fn append(path: impl AsRef<Path>, msg: &str) -> Result<()> {
  let mut logbook = File::options().create(true).append(true).open(path)?;
  writeln!(logbook, "{msg}");
  Ok(())
}



#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[test]
  fn read_returns_none_if_file_does_not_exist() {
    let text = read("tests/data/bogus.txt").unwrap();
    
    assert_eq!(text, None, "expected None");
  }

  #[test]
  fn read_returns_none_if_file_exists_but_is_empty() {
    let text = read("tests/data/empty.txt").unwrap();
    
    assert_eq!(text, None, "expected None");
  }


  #[test]
  fn read_returns_content_of_file_as_string() {
    let text = read("tests/data/logbook.txt").unwrap().unwrap();
    
    assert_eq!(text.trim_end(), "hello world", "expected None");
  }

  #[test]
  fn append_creates_file_if_necessary() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("newlog.txt");
    
    append(&path, "hello logbook").unwrap();
    let text = fs::read_to_string(path).unwrap();
    
    assert_eq!(text, "hello logbook\n", "wrong text");
  }

  #[test]
  fn append_appends_line_to_existing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("newlog.txt");
   
    fs::write(&path, "hello\n").unwrap();

    append(&path, "logbook").unwrap();
    
    let text = fs::read_to_string(path).unwrap();
    
    assert_eq!(text, "hello\nlogbook\n", "wrong text");
  }
}
