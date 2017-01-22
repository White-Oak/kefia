#![feature(box_syntax)]
extern crate regex;
#[macro_use]
extern crate qml;
extern crate lazysort;

use lazysort::*;

use std::process::Command;
use regex::*;

mod view;

pub use view::*;

fn main() {
    let output = Command::new("yaourt")
        .arg("-Qe")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    let hello = ::std::str::from_utf8(&output.stdout).unwrap();

    let re = Regex::new(r"(?m)^(\S+)/(\S+) (\S+)(?: \((.*)\))?$").unwrap();
    let mut pkgs = Vec::new();
    for cap in re.captures_iter(hello) {
        let from = cap.get(4).map_or("", |m| m.as_str());

        pkgs.push(Package {
            name: cap.get(2).map_or("", |m| m.as_str()).into(),
            group: cap.get(1).map_or("", |m| m.as_str()).into(),
            version: cap.get(3).map_or("", |m| m.as_str()).into(),
            meta: from.split_whitespace().map(|s| s.into()).collect(),
            selected: false
        });
    }
    let pkgs: Vec<Package> = pkgs.into_iter().sorted_by(|a, b| a.group.cmp(&b.group)).collect();

    show(pkgs);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub group: String,
    pub version: String,
    pub meta: Vec<String>,
    pub selected: bool,
}

//impl Into<Package> for (String, String, String, String) {
//    fn into(self) -> Package {
//        Package {
//            name: self.0,
//            group: self.1
//            version: self.2
//            meta:
//        }
//    }
//}
