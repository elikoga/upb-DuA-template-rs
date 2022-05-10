/*
In einer Tabelle ist eine (unsortierte) Chronologie von Ereignissen gegeben. In der ersten
Spalte steht eine Uhrzeit in der Form hh:mm:ss ∈ {00:00:00, ... 23:59:59}, in der zweiten
Spalte ein einzeiliger Text.
Sortieren Sie die Tabelle jeweils so, dass die Ereignisse chronologisch sortiert sind (fr ̈uhes
Ereignis zuerst) und bei gleicher Uhrzeit die jeweiligen Texte der zweiten Spalte lexiko-
graphisch (vgl. https://de.wikipedia.org/wiki/Lexikographische Ordnung) sortiert sind.
Die unsortierte Tabelle wird aus einer durch ein Programmargument anzugebenden Datei
gelesen. Jede Zeile der Datei enth ̈alt eine Uhrzeit und einen Text, die durch ein Tabulator-
zeichen getrennt sind. Am Ende kann die Eingabedatei eine leere Zeile enthalten, die zu
ignorieren ist. Die Ausgabe muss genauso formatiert sein wie die Eingabe, d.h. jede Zeile
besteht aus einer Uhrzeit und einem Text, die wieder durch ein Tabulator-Zeichen getrennt
sind.
Beispiel:
11:15:00	aaa
11:15:00	a
11:15:00	ab
09:15:00	z
Ergebnis:
09:15:00	z
11:15:00	a
11:15:00	aaa
11:15:00	ab
*/

use rand::{
    distributions::{DistString, Distribution, Slice},
    prelude::SmallRng,
    Rng, SeedableRng,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

const LINES: [i32; 9] = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

fn main() {
    // read output dir from arg
    let output_dir = std::env::args().nth(1).unwrap();
    // check if is directory, if not exists, create it otherwise fail
    std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        panic!("Error creating output dir: {}", e);
    });
    let outputs: [(fn(i32, BufWriter<File>, BufWriter<File>), &str); 3] = [
        (gen_random, "random"), //hi
        (gen_sorted, "sorted"),
        (gen_reverse, "reversed"),
    ];
    for line_count in LINES {
        for (gen_func, prefix) in &outputs {
            let file_name = format!("{}{}.txt", prefix, line_count);
            // create file in output_dir
            println!(
                "Creating file {}/{{{}, {}.sol}}",
                &output_dir, &file_name, &file_name
            );
            let file = std::fs::File::create(format!("{}/{}", output_dir, file_name)).unwrap();
            let file = std::io::BufWriter::new(file);
            let sol_file =
                std::fs::File::create(format!("{}/{}.sol", output_dir, file_name)).unwrap();
            let sol_file = std::io::BufWriter::new(sol_file);
            println!("Writing to file {}, {}.sol", &file_name, &file_name);
            // generate file
            gen_func(line_count, file, sol_file);
            println!("Done with {}", &file_name);
        }
    }
}

struct CharSlice<'a>(Slice<'a, char>);

impl DistString for CharSlice<'_> {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        let slice = self.0;
        let mut iter = slice.sample_iter(rng);
        for _ in 0..len {
            string.push(*(iter.next().unwrap()));
        }
    }
}

fn random_line(rng: &mut SmallRng) -> String {
    let distribution = Slice::new(&[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
    .unwrap();
    let hour: i32 = rng.gen_range(0..24);
    let minute: i32 = rng.gen_range(0..60);
    let second: i32 = rng.gen_range(0..60);
    let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);
    let text_len = rng.gen_range(1..12);
    let text = CharSlice(distribution).sample_string(rng, text_len);
    format!("{}\t{}", time_str, text)
}

fn gen_random(line_count: i32, mut file: BufWriter<File>, mut sol_file: BufWriter<File>) {
    let mut rng = SmallRng::seed_from_u64(12);
    let mut vec: Vec<String> = Vec::with_capacity(line_count as usize);
    for _ in (0..line_count).by_ref() {
        let line = random_line(&mut rng);
        vec.push(line.clone());
        write!(file, "{}\n", line).unwrap();
    }
    println!("Sorting...");
    vec.sort_unstable();
    for line in vec {
        write!(sol_file, "{}\n", line).unwrap();
    }
}

fn gen_sorted(line_count: i32, mut file: BufWriter<File>, mut sol_file: BufWriter<File>) {
    let mut rng = SmallRng::seed_from_u64(12);
    let mut vec: Vec<String> = Vec::with_capacity(line_count as usize);
    for _ in (0..line_count).by_ref() {
        let line = random_line(&mut rng);
        vec.push(line);
    }
    println!("Sorting...");
    vec.sort_unstable();
    for line in vec {
        write!(file, "{}\n", line).unwrap();
        write!(sol_file, "{}\n", line).unwrap();
    }
}

fn gen_reverse(line_count: i32, mut file: BufWriter<File>, mut sol_file: BufWriter<File>) {
    let mut rng = SmallRng::seed_from_u64(12);
    let mut vec: Vec<String> = Vec::with_capacity(line_count as usize);
    for _ in (0..line_count).by_ref() {
        let line = random_line(&mut rng);
        vec.push(line);
    }
    println!("Sorting...");
    vec.sort_unstable();
    for line in vec.iter().rev() {
        write!(file, "{}\n", line).unwrap();
    }
    for line in vec.iter() {
        write!(sol_file, "{}\n", line).unwrap();
    }
}
