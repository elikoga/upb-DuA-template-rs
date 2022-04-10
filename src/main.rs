use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // last arg is a filename
    let filename = &args[args.len() - 1];
    // open filename
    let contents = std::fs::read_to_string(filename)
        .expect(format!("Error reading file {}", filename).as_str());
    // count content lines
    let lines = contents.lines().count();
    println!("{}", lines);
}
