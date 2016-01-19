extern crate clap;

use clap::*;
use std::process::Command;

fn main() {
    let matches = App::new("gtag")
                             .version("0.1.0")
                             .author("Christoph Burgdorf <christoph@thoughtram.io>")
                             .about("The missing range tag command for Git")
                             .arg(Arg::with_name("from")
                                  .long("from")
                                  .help("Sets the starting point")
                                  .required(true)
                                  .takes_value(true))
                             .arg(Arg::with_name("to")
                                  .long("to")
                                  .help("Sets the ending point")
                                  .required(true)
                                  .takes_value(true))
                             .get_matches();

    let from = matches.value_of("from").unwrap();
    let to   = matches.value_of("to").unwrap();

    println!("{} to {}", from, to);
    get_commits();
}

fn get_commits () {
    let output = Command::new("git")
               .arg("log")
               .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

    let res = String::from_utf8_lossy(&output.stdout);
    println!("{}", res);
}
