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
                              .arg(Arg::with_name("pattern")
                                   .long("pattern")
                                   .help("Sets the pattern for the tag name")
                                   .required(true)
                                   .takes_value(true))
                              .arg(Arg::with_name("dryrun")
                                   .long("dryrun")
                                   .short("d")
                                   .help("Just prints but doesn't tag"))
                             .get_matches();

    let from    = matches.value_of("from").unwrap();
    let to      = matches.value_of("to").unwrap();
    let pattern = matches.value_of("pattern").unwrap();
    let dry_run = matches.is_present("dryrun");

    tag_commits(get_commits(from, to), pattern, dry_run);
}

fn get_commits (from: &str, to: &str) -> Vec<String> {
    let output = Command::new("git")
               .arg("log")
               .arg(from)
               .arg(to)
               .arg("--pretty=oneline")
               .arg("--abbrev-commit")
               .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let res = String::from_utf8_lossy(&output.stdout);
    let res: Vec<String> = res.split('\n')
                              .map(|msg| msg.split(' ').nth(0).unwrap().to_string())
                              .filter(|sha| sha.len() > 0)
                              .collect();
    res
}

fn tag_commits (commits: Vec<String>, pattern: &str, dry_run: bool) {
    for (idx, commit) in commits.iter().enumerate() {
        let tag_name = pattern
                        .replace("##ii", &format!("{}",idx + 1))
                        .replace("##i", &format!("{}",idx));

        if dry_run {
            println!("not tagging commit {} as {} (dry-run)", commit, tag_name);
        }
        else {
            println!("tagging commit {} as {}", commit, tag_name);
            tag_commit(commit, &tag_name);
        }
    }
}

fn tag_commit (commit: &str, tag_name: &str) {
    let output = Command::new("git")
               .arg("tag")
               .arg("-f")
               .arg(tag_name)
               .arg(commit)
               .output().unwrap_or_else(|e| panic!("Failed to run 'git tag' with error: {}", e));

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
