use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use anyhow::Result;
use anyhow::Context;

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

pub fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();

    loop {
        let bytes_read = input.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

pub fn count_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);

    count(file).with_context(|| path.clone())
}

pub fn count_lines(mut input: impl BufRead) -> Result<usize> {
  let mut count = 0;
  let mut line = String::new();

  loop {
    let bytes_read = input.read_line(&mut line)?;

    if bytes_read == 0 {
      break;
    }
    count += 1;
    line.clear();
  }
  Ok(count)
}


pub fn count_words(mut input: impl BufRead) -> Result<usize> {
  let mut count = 0;
  let mut line = String::new();

  loop {
    let bytes_read = input.read_line(&mut line)?;
    if bytes_read == 0 {
      break;
    }
    count += line.split_whitespace().count();
    line.clear();
  }
  Ok(count)
}

#[cfg(test)]
mod tests {
 use super::*;
 use std::io::Cursor;

 #[test]
 fn should_count_lines_and_words() {
   let input = Cursor::new("word1\n word2 word3");
   let count = count(input).unwrap();

   assert_eq!(count.lines, 2, "wrong line count");
   assert_eq!(count.words, 3, "wrong word count");
 }



 #[test]
 fn should_counts_words() {
   let input = Cursor::new("word1 word2 word3");
   let words = count_words(input).unwrap();

   assert_eq!(words, 3, "wrong word count");
 }

 #[test]
 fn should_count_lines() {
   let input = Cursor::new("line1\nline2");
   let lines = count_lines(input).unwrap();

   assert_eq!(lines, 2, "wrong line counts");
 }

}