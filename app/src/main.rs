use std::process;

use add_one;
use structopt::StructOpt;

fn main() {
  let opt = lsclone::Opt::from_args();
  if let Err(ref e) = lsclone::run(&opt.path) {
      println!("{}", e);
      process::exit(1);
  }
}