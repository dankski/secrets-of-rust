use anyhow::Result;

use std::env;

use memo::{open, sync};


fn main() -> Result<()> {
  let mut memos: Vec<String> = open("memo.txt")?;
  let args: Vec<_> = env::args().skip(1).collect();

  if args.is_empty() {
    for memo in &memos {
      println!("{memo}");
    }
  } else {
    let memo = args.join(" ");
    memos.push(memo);
    sync(&memos, "memo.txt")?;
  }
  Ok(())
}
