use clap::{App, Arg};
use difr::Difr;
use std::path::PathBuf;

fn main() {
    let matches = App::new("difr")
        .version("0.1.0")
        .author("0xba1")
        .about("Compares two text files for differences.")
        .arg(
            Arg::new("file1")
                .about("Sets one of the files to compare")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("file2")
                .about("Sets the file path of one of the files to compare")
                .required(true)
                .index(2),
        )
        .get_matches();

    let file1 = matches.value_of("file1").unwrap();
    let file2 = matches.value_of("file2").unwrap();

    let mut app = Difr::init(
        PathBuf::from(file1),
        PathBuf::from(file2),
        false,
        None,
        None,
    );
    app.run();
}
