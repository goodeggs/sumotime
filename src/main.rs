extern crate getopts;
extern crate wait_timeout;
#[macro_use]
extern crate version;

use std::process;
use std::process::Command;
use wait_timeout::ChildExt;
use getopts::Options;
use std::env;
use std::time::{Duration, SystemTime};

fn main() {

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("k", "key", "timing key (required)", "KEY");
  opts.optopt("t", "timeout", "optional timeout in seconds, after which we'll SIGKILL", "TIMEOUT");
  opts.optflag("v", "version", "print the version");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => {
      print!("{}", f.to_string());
      process::exit(1);
    }
  };

  if matches.opt_present("v") {
    print!("{}\n", version!());
    return;
  }

  if matches.opt_present("h") {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    return;
  }

  if !matches.opt_present("k") {
    print!("ERROR: -k/--key is required\n\n");
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(1);
  }

  let key = matches.opt_str("k").unwrap();

  let command = &matches.free[0];
  let args = &matches.free[1..];

  let now = SystemTime::now();

  let mut child = Command::new(command).args(&args).spawn().unwrap();
  
  if matches.opt_present("t") {
    let timeout = matches.opt_str("t").unwrap().parse::<u64>().unwrap();
    let timeout_duration = Duration::from_secs(timeout);
    match child.wait_timeout(timeout_duration).unwrap() {
      Some(status) => finalize(key, status.code().unwrap(), false, now),
      None => {
        // child hasn't exited yet
        let _ = child.kill();
        let _ = child.wait();
        finalize(key, 124, true, now);
      }
    };
  } else {
    finalize(key, child.wait().unwrap().code().unwrap(), false, now);
  }

}

fn finalize(key: String, exit_code: i32, was_timeout: bool, started_at: SystemTime) {
  let duration = started_at.elapsed().unwrap().as_secs();
  print!("key={} code={} timeout={} duration={}", key, exit_code, was_timeout, duration);
  process::exit(exit_code);
}
 
