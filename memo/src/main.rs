use anyhow::Result;

use std::env;

use memo::{open, sync};

fn main() -> Result<()> {
  let mut memos = open("memo.txt");
  let args: Vec<_> = env::args().skip(1).collect();

  if args.is_empty() {
    for memo in &memos {
      println!("{memo}");
    }
  } else {
    let memo = args.join(" ");
    memos.push(memo);
    sync(&memo, "memo.txt")?;
  }
  Ok(())
}
