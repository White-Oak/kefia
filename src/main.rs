extern crate regex;

use std::process::Command;
use regex::*;

fn main() {
    let output = Command::new("yaourt")
        .arg("-Qe")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let hello = ::std::str::from_utf8(&output.stdout).unwrap();

    let re = Regex::new(r"(?m)^(\S+)/(\S+) (\S+)(?: \((.*)\))?$").unwrap();
    let mut pkgs = Vec::new();
    for cap in re.captures_iter(hello) {
        let from = cap.at(4).unwrap_or("");

        pkgs.push(Package {
            name: cap.at(2).unwrap_or(""),
            group: cap.at(1).unwrap_or(""),
            version: cap.at(3).unwrap_or(""),
            meta: from.split(' ').collect(),
        });
    }

    for pkg in pkgs {
        println!("{:?}", pkg)
    }
}

#[derive(Debug)]
struct Package<'a> {
    name: &'a str,
    group: &'a str,
    version: &'a str,
    meta: Vec<&'a str>,
}
