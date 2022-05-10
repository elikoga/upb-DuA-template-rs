use std::io::Write;

use rand::{prelude::SmallRng, Rng, SeedableRng};

#[cfg(test)]
mod tests {
    fn strip_carriage_return(s: &str) -> String {
        s.replace("\r", "")
    }
    #[test]
    fn pub_inst() {
        // execute ./target/release/aufgabe01 on files in ./pubInst
        // read folder first
        let generated_tests = std::fs::read_dir("./generatedTests")
            .map(|dir| dir.collect::<Vec<_>>())
            .unwrap_or(vec![]);
        let pub_inst = std::fs::read_dir("./pubInst")
            .map(|dir| dir.collect::<Vec<_>>())
            .unwrap_or(vec![]);
        let all = generated_tests
            .iter()
            .chain(pub_inst.iter())
            .collect::<Vec<_>>();
        // filter ending with .txt
        let test_files = all
            .iter()
            .filter(move |f| f.as_ref().unwrap().path().extension().unwrap() == "txt");

        let tests = test_files.map(|f| {
            let path = f.as_ref().map_err(|e| e.to_string())?.path();
            let test_file = path.to_str().ok_or("Error reading path")?.to_string();
            let sol_file = format!("{}.sol", test_file);
            let ret: Result<(String, String), String> = Ok((test_file, sol_file));
            ret
        });
        // sort tests by test_file
        let mut tests = tests.map(|t| t.unwrap()).collect::<Vec<_>>();
        tests.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?} tests", tests);
        // for each test, execute ./target/release/aufgabe01 -quick on test_file and compare with sol_files
        for test in tests {
            let (test_file, sol_file) = test;
            let result = std::process::Command::new("./target/release/aufgabe01")
                .arg("-quick")
                .arg(&test_file)
                .output()
                .expect("failed to execute process");
            let result = String::from_utf8(result.stdout).unwrap();
            // strip \r
            let result = strip_carriage_return(&result);
            // read sol_file
            let sol_content = std::fs::read_to_string(&sol_file).unwrap();
            // strip \r
            let sol_content = strip_carriage_return(&sol_content);
            // compare with sol_file
            assert_eq!(sol_content, result, "Error in test file {}", test_file);
            // same for -merge
            let result = std::process::Command::new("./target/release/aufgabe01")
                .arg("-merge")
                .arg(&test_file)
                .output()
                .expect("failed to execute process");
            let result = String::from_utf8(result.stdout).unwrap();
            let result = strip_carriage_return(&result);
            assert_eq!(sol_content, result, "Error in test file {}", test_file);
        }
    }
}

pub fn print_vec(vec: &Vec<String>) {
    let buffered_stdout = std::io::stdout();
    let mut buffered_stdout = std::io::BufWriter::new(buffered_stdout);
    for line in vec {
        buffered_stdout
            .write_all(line.as_bytes())
            .expect("Error writing to stdout");
        buffered_stdout
            .write_all("\n".as_bytes())
            .expect("Error writing to stdout");
    }
    // construct a string with all elements of vec
    // and print it
    // let out = vec.join("\n");
    // println!("{}", out);
}

pub fn quicksort(vec: &mut Vec<String>) {
    let mut rng = SmallRng::seed_from_u64(123);
    quicksort_rec(vec, 0, vec.len() - 1, &mut rng);
    print_vec(vec);
}

fn quicksort_rec(vec: &mut Vec<String>, start: usize, end: usize, rng: &mut SmallRng) {
    // if we don't have anything to sort, we're done
    if start >= end {
        return;
    }
    // if we have less than CUTOFF elements, we sort them with insertion sort
    if end - start < CUTOFF {
        insertion_sort(vec, start, end);
        return;
    }
    let pivot = partition(vec, start, end, rng);
    if pivot != 0 {
        quicksort_rec(vec, start, pivot - 1, rng);
    }
    quicksort_rec(vec, pivot + 1, end, rng);
}

fn partition(vec: &mut Vec<String>, start: usize, end: usize, rng: &mut SmallRng) -> usize {
    let pivot = rng.gen_range(start..end);
    vec.swap(pivot, end);
    let pivot = (&vec[end]).to_string();
    let mut i = start;
    for j in start..end {
        if vec[j] < pivot {
            vec.swap(i, j);
            i += 1;
        }
    }
    vec.swap(i, end);
    i
}

pub fn mergesort(vec: &mut Vec<String>) {
    // https://www.cs.princeton.edu/courses/archive/spr14/cos226/lectures/22Mergesort.pdf
    // optimizations from here
    let mut helper = vec.to_vec();
    mergesort_rec(&mut helper, vec, 0, vec.len() - 1);
    print_vec(vec);
}

const CUTOFF: usize = 10;

pub fn mergesort_rec(vec: &mut Vec<String>, helper: &mut Vec<String>, start: usize, end: usize) {
    // sort from vec -> helper
    // if we don't have anything to sort, we're done
    if start >= end {
        return;
    }
    // if we have less than CUTOFF elements, we sort them with insertion sort
    if end - start < CUTOFF {
        insertion_sort(helper, start, end);
        return;
    }
    // sort from helper -> vec
    let mid = start + (end - start) / 2;
    mergesort_rec(helper, vec, start, mid);
    mergesort_rec(helper, vec, mid + 1, end);
    // if the two halves are already sorted, we're done
    if vec[mid] <= vec[mid + 1] {
        // copy from vec -> helper
        // helper[start..=end].clone_from_slice(&vec[start..=end]);
        for i in start..=end {
            // mem::swap(&mut vec[i], &mut helper[i]);
            helper[i] = vec[i].clone();
        }
        return;
    }
    // merge from vec -> helper
    merge(vec, helper, start, mid, end);
}

pub fn merge(
    helper: &mut Vec<String>,
    vec: &mut Vec<String>,
    start: usize,
    mid: usize,
    end: usize,
) {
    // copy vec to helper
    // helper[start..=end].clone_from_slice(&vec[start..=end]);
    let mut i = start;
    let mut j = mid + 1;
    let mut k = start;
    while k <= end {
        if i > mid {
            vec[k] = helper[j].clone();
            // mem::swap(&mut vec[k], &mut helper[j]);
            j += 1;
        } else if j > end {
            vec[k] = helper[i].clone();
            // mem::swap(&mut vec[k], &mut helper[i]);
            i += 1;
        } else if helper[i] < helper[j] {
            vec[k] = helper[i].clone();
            // mem::swap(&mut vec[k], &mut helper[i]);
            i += 1;
        } else {
            vec[k] = helper[j].clone();
            // mem::swap(&mut vec[k], &mut helper[j]);
            j += 1;
        }
        k += 1;
    }
}

fn insertion_sort(vec: &mut Vec<String>, start: usize, end: usize) {
    for i in start..=end {
        let mut j = i;
        while j > start && vec[j - 1] > vec[j] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}
