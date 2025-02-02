use std::cmp::min;

use rbmstable_parser::parse;

/// Print difficult table's meta info and its first 20 songs
fn main() {
    let url = "http://zris.work/bmstable/satellite/header.json";
    let dth = parse(url.to_string()).unwrap();
    println!(
        "[{}|{}] collects {} songs and {} courses",
        dth.name,
        dth.symbol,
        dth.contents.len(),
        dth.courses.len()
    );

    for i in 0..min(20, dth.contents.len()) {
        println!(
            "{i}th song's title is {}, md5 is {}",
            dth.contents[i].title, dth.contents[i].md5
        );
    }
}
