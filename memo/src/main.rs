use anyhow::Result;

use std::env;

use memo::Memos;


fn main() -> Result<()> {
  let mut memos = Memos::open("memo.txt")?;
  let args: Vec<_> = env::args().skip(1).collect();

  if args.is_empty() {
    for memo in &memos.inner {
      println!("{memo}");
    }
  } else {
    let memo = args.join(" ");
    memos.inner.push(memo);
    memos.sync()?;
  }
  Ok(())
}
