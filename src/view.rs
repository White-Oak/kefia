use qmlrs::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

use super::Package;

pub fn show(gathered: &[Package]) {
    save_data(gathered).unwrap();
    let mut engine = Engine::new();

    engine.load_local_file("view.qml");

    engine.exec();
}

fn save_data(gathered: &[Package]) -> Result<(), Error> {
    let mut list = String::new();
    for pkg in gathered {
        list = list +
               &format!(r#"ListElement{{
            name: "{}"
            version: "{}"
            group: "{}"
            supdupsecret: [{}]
        }}
        "#,
                        pkg.name,
                        pkg.version,
                        pkg.group,
                        pkg.meta
                            .iter()
                            .map(|s| format!("ListElement{{ one: \"{}\" }}", s))
                            .collect::<Vec<String>>()
                            .join(","));
    }
    let data = include_str!("view.qml").replace("LISTPKGS", &list);
    let mut f = try!(File::create("view.qml"));
    try!(f.write_all(data.as_bytes()));
    Ok(())
}
