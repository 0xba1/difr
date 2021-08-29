use colored::*;
use sha3::{Digest, Sha3_256};
use std::{
    fs::{metadata, read_to_string},
    io::Read,
    path::PathBuf,
};

pub struct Difr {
    // `PathBuf` of one of the two files to be compared
    first_file_path: PathBuf,
    // Content of the `first_file_path`
    first_file: String,
    // `PathBuf` of one of the two files to be compared
    second_file_path: PathBuf,
    // Content of the `second_file_path`
    second_file: String,
    // Whether to exclude empty lines or not
    exclude_empty_lines: bool,
    // Line index to comparison begins. [Optional]
    from: Option<usize>,
    // Line index to comparison ends. [Optional]
    to: Option<usize>,
}

impl Difr {
    pub fn init(
        first_file_path: PathBuf,
        second_file_path: PathBuf,
        exclude_empty_lines: bool,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Self {
        // Content of first_file
        let first_file = read_to_string(&first_file_path)
            .expect(format!("Unable to read file: {:?}", &first_file_path).as_str());
        // Content of second_file file
        let second_file = read_to_string(&second_file_path)
            .expect(format!("Unable to read file: {:?}", &second_file_path).as_str());

        Difr {
            first_file_path,
            first_file,
            second_file_path,
            second_file,
            exclude_empty_lines,
            from,
            to,
        }
    }

    pub fn run(&self) {
        // Size of first_file in bytes
        let size1 = metadata(&self.first_file_path)
            .expect("Unable to get first file metadata")
            .len();
        // Size of second_file in bytes
        let size2 = metadata(&self.second_file_path)
            .expect("Unable to get second file metadata")
            .len();
        // No of lines in first_file
        let no_of_lines1 = &self.first_file.lines().count();
        // No of lines in first_file
        let no_of_lines2 = &self.second_file.lines().count();

        // """File 1:    file_name    size       no_of_lines"""
        println!(
            "{}:\t{}\t{}{}\t{}{}",
            "File 1".bright_green(),
            &self.first_file_path.to_string_lossy().bright_blue(),
            &size1.to_string().bright_blue(),
            "bytes".bright_green(),
            &no_of_lines1.to_string().bright_blue(),
            "lines".bright_green()
        );
        // """File 2:    file_name    size       no_of_lines"""
        println!(
            "{}:\t{}\t{}{}\t{}{}",
            "File 2".bright_green(),
            &self.second_file_path.to_string_lossy().bright_blue(),
            &size2.to_string().bright_blue(),
            "bytes".bright_green(),
            &no_of_lines2.to_string().bright_blue(),
            "lines".bright_green()
        );

        println!(
            "\n{}",
            "Computing SHA3-256 hashes of files...".bright_green()
        );
        let file1_hash = Difr::hash(&self.first_file);
        let file2_hash = Difr::hash(&self.second_file);

        println!(
            "{}:\t{}",
            "File 1".bright_green(),
            &file1_hash.bright_blue()
        );
        println!(
            "{}:\t{}",
            "File 2".bright_green(),
            &file2_hash.bright_blue()
        );

        if file1_hash == file2_hash {
            println!(
                "\n{}\n",
                "Contents of files are equal (hashes are equal)".bright_green()
            );
        } else {
            println!("\n{}\n", "Contents of files are different".bright_red());
        }
    }

    pub fn difr_include_empty_lines(&self) {}

    pub fn difr_exclude_empty_lines(&self) {
        // Content of first_file
        let first_content = read_to_string(&self.first_file)
            .expect(format!("Unable to read file: {:?}", &self.first_file).as_str());
        // Content of second_file file
        let second_content = read_to_string(&self.second_file)
            .expect(format!("Unable to read file: {:?}", &self.second_file).as_str());
    }

    fn is_text(pathbuf: PathBuf) -> bool {
        let file = std::fs::File::open(pathbuf).expect("failed to open file");

        let mut buffer = Vec::with_capacity(32);
        file.take(32)
            .read_to_end(&mut buffer)
            .expect("failed to read from file");

        match std::str::from_utf8(&buffer) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn hash(file_string: &str) -> String {
        let mut hasher = Sha3_256::new();

        // Write input message
        hasher.update(file_string.as_bytes());

        // Read hash digest
        let result = hasher.finalize();
        result.iter().fold(String::new(), |acc, cur_num| {
            format!("{}{:x}", acc, cur_num)
        })
    }
}
