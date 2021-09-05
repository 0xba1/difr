use clap::{App, Arg};
use difr::Difr;
use std::path::PathBuf;

fn main() {
    let matches = App::new("difr")
        .version("0.1.0")
        .author("0xba1")
        .about("Compares two text files for differences. (For other file types, it only compares its sha3-256 hashes for equality)")
        .arg(Arg::new("file1")
            .about("Sets one of the files to compare")
            .required(true)
            .index(1))
        .arg(Arg::new("file2")
            .about("Sets the file path of one of the files to compare")
            .required(true)
            .index(2))
        .get_matches();

    let file1 = matches.value_of("file1").unwrap();
    let file2 = matches.value_of("file2").unwrap();

    let exclude_empty_lines = matches.value_of("exclude_empty_lines");
    println!("{:?}\n{}\n{}", exclude_empty_lines, &file1, &file2);

    let mut app = Difr::init(
        PathBuf::from(file1),
        PathBuf::from(file2),
        false,
        None,
        None,
    );
    app.run();
}
