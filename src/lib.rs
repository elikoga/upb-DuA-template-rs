#[cfg(test)]
mod tests {
    #[test]
    fn pub_inst() {
        // execute ./target/release/aufgabe00 on files in ./pubInst
        // read folder first
        let files = std::fs::read_dir("./pubInst").unwrap();
        // filter ending with .txt
        let test_files =
            files.filter(move |f| f.as_ref().unwrap().path().extension().unwrap() == "txt");
        // for each test_file, we also have 2 solution files. They end with ..sol and .np.sol respectively
        let tests = test_files.map(|f| {
            let path = f.map_err(|e| e.to_string())?.path();
            let test_file = path.to_str().ok_or("Error reading path")?.to_string();
            let sol_file = format!("{}..sol", test_file);
            let np_sol_file = format!("{}.np.sol", test_file);
            let ret: Result<(String, String, String), String> =
                Ok((test_file, sol_file, np_sol_file));
            ret
        });
        // for each test, execute ./target/release/aufgabe00 on test_file and compare with sol_files
        for test in tests {
            let (test_file, sol_file, np_sol_file) = test.unwrap();
            let result = std::process::Command::new("./target/release/aufgabe00")
                .arg(&test_file)
                .output()
                .expect("failed to execute process");
            // read sol_file
            let sol_content = std::fs::read_to_string(&sol_file).unwrap();
            // compare with sol_file
            assert_eq!(
                sol_content,
                String::from_utf8_lossy(&result.stdout).to_string(),
                "Error in test file {}",
                test_file
            );
            // again with -np
            let np_result = std::process::Command::new("./target/release/aufgabe00")
                .arg("-np")
                .arg(&test_file)
                .output()
                .expect("failed to execute process");
            // read np_sol_file
            let np_sol_content = std::fs::read_to_string(&np_sol_file).unwrap();
            // compare with np_sol_file
            assert_eq!(
                np_sol_content,
                String::from_utf8_lossy(&np_result.stdout).to_string(),
                "Error in test file {} with -np",
                test_file
            );
        }
    }
}

pub fn sieve(n: i64, non_primes: bool) {
    // print all primes or non-primes from 2 up to including n
    let mut is_prime = vec![true; n as usize + 1]; // +1 for 0-based indexing
    is_prime[0] = false; // 0 is not a prime
    is_prime[1] = false; // 1 is not a prime
    for i in 2..(n as f64)
        .sqrt() // we only need to check up to sqrt(n)
        .ceil() // ceil to get the next integer
        as i64
    {
        if is_prime[i as usize] {
            // if i is a prime
            // mark multiples of i as non-prime
            for j in (i * i..=n) // start at i^2 and stop at n
                .step_by(
                    i as usize, // step by i
                )
            {
                is_prime[j as usize] = false; // mark as non-prime
            }
        }
    }
    // print
    for (i, p) in is_prime.iter().enumerate().skip(2) {
        // skip 0 and 1
        if *p != non_primes {
            // print out primes or non-primes depending on -np
            println!("{}", i); // print i
        }
    }
}
