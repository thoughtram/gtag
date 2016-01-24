extern crate clap;

use clap::*;
use std::process::Command;

fn main() {
    let matches = App::new("gtag")
                             .version("0.2.1")
                             .author("Christoph Burgdorf <christoph@thoughtram.io>")
                             .about("The missing range tag command for Git")
                             .arg(Arg::with_name("range")
                                 .help("Sets the commit range")
                                 .required(true)
                                 .index(1))
                             .arg(Arg::with_name("pattern")
                                 .help("Sets the pattern for the tag name")
                                 .index(2)
                                 .required(true))
                             .arg(Arg::with_name("dryrun")
                                 .long("dryrun")
                                 .short("d")
                                 .help("Just prints but doesn't tag"))
                             .get_matches();

    let range   = matches.value_of("range").unwrap();
    let pattern = matches.value_of("pattern").unwrap();
    let dry_run = matches.is_present("dryrun");

    println!("{}", range);
    tag_commits(get_commits(range), pattern, dry_run);
}

fn get_commits (range: &str) -> Vec<String> {
    let output = Command::new("git")
               .arg("log")
               .arg(range)
               .arg("--reverse")
               .arg("--boundary")
               .arg("--pretty=oneline")
               .arg("--abbrev-commit")
               .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    let res = String::from_utf8_lossy(&output.stdout);
    let res: Vec<String> = res.split('\n')
                              //--boundary places a `- ` infront of the commit hash, hence the replace
                              .map(|msg| msg.replace("- ", "").split(' ').nth(0).unwrap().to_string())
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
