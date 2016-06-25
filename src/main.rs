extern crate regex;
extern crate qmlrs;

use std::process::Command;
use regex::*;

mod view;

use view::*;

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
    show(&pkgs);
}

#[derive(Debug)]
pub struct Package<'a> {
    pub name: &'a str,
    pub group: &'a str,
    pub version: &'a str,
    pub meta: Vec<&'a str>,
}
