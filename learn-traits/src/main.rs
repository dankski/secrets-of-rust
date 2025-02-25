use learn_traits::log;
use learn_traits::utils;
use learn_traits::helper;

fn write_helper(log: &impl helper::Writer, msg: &String) {
  log.write(&msg);
}

fn write_utils(log: &impl utils::Writer, msg: &String) {
  log.write(&msg);
}

fn main() {
  let logger = log::Logger{};
  logger.write(&"Hello".to_owned());
  write_helper(&logger, &String::from("Hello"));
  write_utils(&logger, &String::from("Hello"));
}