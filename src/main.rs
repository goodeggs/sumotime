extern crate getopts;
extern crate wait_timeout;
#[macro_use]
extern crate version;
extern crate hyper;
extern crate hyper_native_tls;

use std::io::Read;
use std::process;
use std::process::Command;
use wait_timeout::ChildExt;
use getopts::Options;
use std::env;
use std::time::{Duration, SystemTime};
use hyper::client::Client;
use hyper::header::{Accept, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn main() {

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("u", "url", "URL to POST result to (required, also $SUMOTIME_URL)", "URL");
  opts.optopt("k", "key", "timing key (required)", "KEY");
  opts.optopt("t", "timeout", "optional timeout in seconds, after which we'll SIGKILL", "TIMEOUT");
  opts.optflag("v", "version", "print the version");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => {
      println!("{}", f.to_string());
      process::exit(1);
    }
  };

  if matches.opt_present("v") {
    println!(version!());
    return;
  }

  if matches.opt_present("h") {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    return;
  }

  if !matches.opt_present("k") {
    println!("ERROR: -k/--key is required\n");
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(1);
  }

  let key = matches.opt_str("k").unwrap();

  let url: String = match matches.opt_str("u") {
    Some(v) => v,
    None => {
      match env::var("SUMOTIME_URL") {
        Ok(v) => v,
        Err(_) => {
          println!("ERROR: -u/--url or $SUMOTIME_URL is required\n");
          let brief = format!("Usage: {} [options]", program);
          print!("{}", opts.usage(&brief));
          process::exit(1);
        }
      }
    }
  };

  let command = &matches.free[0];
  let args = &matches.free[1..];

  let now = SystemTime::now();

  let mut child = Command::new(command).args(&args).spawn().unwrap();
  
  if matches.opt_present("t") {
    let timeout = matches.opt_str("t").unwrap().parse::<u64>().unwrap();
    let timeout_duration = Duration::from_secs(timeout);
    match child.wait_timeout(timeout_duration).unwrap() {
      Some(status) => finalize(url, key, status.code().unwrap(), false, now),
      None => {
        // child hasn't exited yet
        let _ = child.kill();
        let _ = child.wait();
        finalize(url, key, 124, true, now);
      }
    };
  } else {
    finalize(url, key, child.wait().unwrap().code().unwrap(), false, now);
  }

}

fn finalize(url: String, key: String, exit_code: i32, was_timeout: bool, started_at: SystemTime) {
  let duration = started_at.elapsed().unwrap().as_secs();
  let body = format!(r#"{{"msg":"sumotime","key":"{}","code":{},"timeout":{},"duration":{}}}"#, key, exit_code, was_timeout, duration);

  let ssl = NativeTlsClient::new().unwrap();
  let connector = HttpsConnector::new(ssl);
  let client = Client::with_connector(connector);
  match client
    .post(url.as_str())
    .body(&body)
    .header(ContentType::json())
    .header(Accept::json())
    .send() {
    Ok(mut res) => {
      if !res.status.is_success() {
        println!("sumotime WARN: {}", res.status);
        let mut res_body = String::new();
        match res.read_to_string(&mut res_body) {
          Ok(_) => println!("{}", res_body),
          Err(e) => println!("error reading body: {:?}", e)
        };
      }
    }
    Err(e) => println!("sumotime Err: {:?}", e)
  };
  process::exit(exit_code);
}
 
