use colored::*;
use sha3::{Digest, Sha3_256};
use std::{
    fs::{metadata, read_to_string},
    io::Read,
    path::PathBuf,
};

/// The Cli App instance
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
    /// Initializes the `Difr` app with:
    /// first_file_path: PathBuf,
    /// first_file: String,
    /// second_file_path: PathBuf,
    /// second_file: String,
    /// exclude_empty_lines: bool,
    /// from: Option<usize>,
    /// to: Option<usize>,
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

    /// Runs the `Difr` app
    pub fn run(&mut self) {
        // Prints info about the two files such as
        // file name, file size, number of lines
        self.info();
        // Prints and compares hashes of files
        self.compare_hashes();
        // Compares content of both files
        self.difr_include_empty_lines();
    }

    // Displays information about both files
    pub fn info(&self) {
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
            &self.first_file_path.to_string_lossy().bright_cyan(),
            &size1.to_string().bright_cyan(),
            "bytes".bright_cyan(),
            &no_of_lines1.to_string().bright_cyan(),
            "lines".bright_cyan()
        );
        // """File 2:    file_name    size       no_of_lines"""
        println!(
            "{}:\t{}\t{}{}\t{}{}",
            "File 2".bright_green(),
            &self.second_file_path.to_string_lossy().bright_cyan(),
            &size2.to_string().bright_cyan(),
            "bytes".bright_cyan(),
            &no_of_lines2.to_string().bright_cyan(),
            "lines".bright_cyan()
        );
    }

    // Prints and compares hashes of both files
    pub fn compare_hashes(&self) -> bool {
        println!(
            "\n{}",
            "Computing SHA3-256 hashes of files...".bright_green()
        );
        let file1_hash = Difr::hash(&self.first_file);
        let file2_hash = Difr::hash(&self.second_file);

        println!(
            "{}:\t{}",
            "File 1".bright_green(),
            &file1_hash.bright_cyan()
        );
        println!(
            "{}:\t{}",
            "File 2".bright_green(),
            &file2_hash.bright_cyan()
        );

        if file1_hash == file2_hash {
            println!(
                "\n{}\n",
                "Contents of files are equal (hashes are equal)".bright_green()
            );
            true
        } else {
            println!("\n{}\n", "Contents of files are different".bright_cyan());
            false
        }
    }

    // Compares content of both files
    pub fn difr_include_empty_lines(&mut self) {
        // `Lines` iter of lines of each file string content
        let lines1 = &mut self.first_file.lines();
        let lines2 = &mut self.second_file.lines();

        // Current line number of both `Line` `iter`s
        let mut current_line_no: usize = 1;

        // Go through the iter of `lines1` while going through that of `lines2` and compare them
        while let Some(current_line1) = lines1.next() {
            match &lines2.next() {
                // When lines2 has fewer lines than lines1 and has reached the cut-off
                None => {
                    println!(
                        "{} {} {}:\n{}\t{}",
                        "Lines".bright_green(),
                        &current_line_no.to_string().bright_green(),
                        "-> End".bright_green(),
                        ">".bright_green(),
                        current_line1.bright_cyan()
                    );
                    while let Some(lines1_extra_line) = lines1.next() {
                        println!("\t{}", lines1_extra_line.bright_cyan());
                    }
                    println!(
                        "{}\t{}`{}`",
                        ">".bright_green(),
                        "End of File for ".bright_cyan(),
                        &self.second_file_path.to_string_lossy().bright_cyan()
                    );
                }

                Some(current_line2) => {
                    if &current_line1 == current_line2 {
                        current_line_no += 1;
                        continue;
                    } else {
                        println!(
                            "{} {}:\n{}\t{}\n{}\t{}\n",
                            "Line".bright_green(),
                            &current_line_no.to_string().bright_green(),
                            ">".bright_green(),
                            &current_line1.bright_cyan(),
                            ">".bright_green(),
                            &current_line2.bright_cyan()
                        );
                        current_line_no += 1;
                    }
                }
            }
        }

        // When `lines1` has fewer lines than `lines2`
        if let Some(lines2_extra_line) = lines2.next() {
            println!(
                "{} {} {}:\n{}\t{}`{}`\n{}\t{}",
                "Lines".bright_green(),
                &current_line_no.to_string().bright_green(),
                "-> End".bright_green(),
                ">",
                "End of File for ".bright_cyan(),
                &self.first_file_path.to_string_lossy().bright_cyan(),
                ">".bright_green(),
                lines2_extra_line.bright_cyan()
            );
        }
        while let Some(lines2_extra_line) = lines2.next() {
            println!("\t{}", lines2_extra_line.bright_cyan());
        }
    }

    pub fn difr_exclude_empty_lines(&self) {
        // `Lines` iter of lines of each file string content
        let lines1 = &mut self
            .first_file
            .lines()
            .filter(|curr_line| !curr_line.is_empty());
        let lines2 = &mut self
            .second_file
            .lines()
            .filter(|curr_line| !curr_line.is_empty());

        // Current line number of both `Line` `iter`s
        let mut current_line_no: usize = 1;

        // Go through the iter of `lines1` while going through that of `lines2` and compare them
        while let Some(current_line1) = lines1.next() {
            match &lines2.next() {
                // When lines2 has fewer lines than lines1 and has reached the cut-off
                None => {
                    println!(
                        "{} {} {}:\n{}\t{}",
                        "Adjusted lines".bright_green(),
                        &current_line_no.to_string().bright_green(),
                        "-> End".bright_green(),
                        ">".bright_green(),
                        current_line1.bright_cyan()
                    );
                    while let Some(lines1_extra_line) = lines1.next() {
                        println!("\t{}", lines1_extra_line.bright_cyan());
                    }
                    println!(
                        "{}\t{}`{}`",
                        ">".bright_green(),
                        "End of File for ".bright_cyan(),
                        &self.second_file_path.to_string_lossy().bright_cyan()
                    );
                }

                Some(current_line2) => {
                    if &current_line1 == current_line2 {
                        current_line_no += 1;
                        continue;
                    } else {
                        println!(
                            "{} {}:\n{}\t{}\n{}\t{}\n",
                            "Adjusted line".bright_green(),
                            &current_line_no.to_string().bright_green(),
                            ">".bright_green(),
                            &current_line1.bright_cyan(),
                            ">".bright_green(),
                            &current_line2.bright_cyan()
                        );
                        current_line_no += 1;
                    }
                }
            }
        }

        // When `lines1` has fewer lines than `lines2`
        if let Some(lines2_extra_line) = lines2.next() {
            println!(
                "{} {} {}:\n{}\t{}`{}`\n{}\t{}",
                "Adjusted lines".bright_green(),
                &current_line_no.to_string().bright_green(),
                "-> End".bright_green(),
                ">",
                "End of File for ".bright_cyan(),
                &self.first_file_path.to_string_lossy().bright_cyan(),
                ">".bright_green(),
                lines2_extra_line.bright_cyan()
            );
        }
        while let Some(lines2_extra_line) = lines2.next() {
            println!("\t{}", lines2_extra_line.bright_cyan());
        }
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

        // Convert result from array to hex string
        hex::encode(result)
    }
}
