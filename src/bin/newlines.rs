use std::env;

fn main() {
    // in a folder passed on the cli, take all filenames
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];
    let files = std::fs::read_dir(dir).unwrap();
    let files = files.map(|f| f.unwrap().path());
    let files = files.map(|f| f.to_str().unwrap().to_string());
    let mut files = files.collect::<Vec<_>>();
    files.sort();
    // for each file
    for file in files.clone() {
        // check if it contains CRLF (\r\n) instead of just \n
        let contents = std::fs::read_to_string(file.clone()).unwrap();
        let contains = contents.contains("\r\n");
        if contains {
            println!("{}: contains CRLF", file.clone());
        } else {
            println!("{}: contains only LF !!!", file.clone());
        }
    }
}
