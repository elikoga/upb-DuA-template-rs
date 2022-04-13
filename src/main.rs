use std::env;

use aufgabe00::sieve;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if first argument is -np, set np flag
    let non_primes = args.len() > 1 && args[1] == "-np";
    // last arg is a filename
    let filename = &args[args.len() - 1];
    // open filename
    let contents = std::fs::read_to_string(filename)
        .expect(format!("Error reading file {}", filename).as_str());
    // read file and parse as int
    let n: i64 = contents.trim().parse().unwrap();
    // execute sieve
    sieve(n, non_primes);
}
