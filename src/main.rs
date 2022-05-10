use std::env;

use aufgabe01::{mergesort, quicksort};

fn main() {
    let args: Vec<String> = env::args().collect();
    // set sorter depending on first arg: either -quick or -merge
    let sorter = match args.get(1).map(|s| s.as_str()) {
        Some("-quick") => quicksort,
        Some("-merge") => mergesort,
        _ => {
            println!("Usage: {} [-quick|-merge]", args[0]);
            return;
        }
    };
    // last arg is a filename
    let filename = &args[args.len() - 1];
    // open filename
    let contents = std::fs::read_to_string(filename)
        .expect(format!("Error reading file {}", filename).as_str());
    // split into lines
    let lines: Vec<&str> = contents.split("\n").collect();
    // convert lines to vec of strings
    let mut vec: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    // if last line is empty, remove it
    if vec.last().map(|s| s.is_empty()).unwrap_or(false) {
        vec.pop();
    }
    // sort vec
    sorter(&mut vec);
}
