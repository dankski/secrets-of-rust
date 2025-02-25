use crate::utils;
use crate::helper;

pub struct Logger {
}

impl utils::Writer for Logger {
  fn write(&self, msg: &String) {
    println!("{msg}");
  }
}

impl helper::Writer for Logger {
  fn write(&self, msg: &String) {
    println!("H: {msg}");
  }
}
