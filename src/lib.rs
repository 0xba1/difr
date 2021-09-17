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
    //exclude_empty_lines: bool,
    // Line index to comparison begins. [Optional]
    //from: Option<usize>,
    // Line index to comparison ends. [Optional]
    //to: Option<usize>,
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
        //exclude_empty_lines: bool,
        //from: Option<usize>,
        //to: Option<usize>,
    ) -> Self {
        if !Difr::is_text(&first_file_path) || !Difr::is_text(&second_file_path) {
            println!(
                "{}",
                "This program only compares text files for now, check for latest version."
                    .bright_green()
            );
            std::process::exit(0);
        }

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
            //exclude_empty_lines,
            //from,
            //to,
        }
    }

    /// Runs the `Difr` app
    pub fn run(&mut self) {
        // Prints info about the two files such as
        // file name, file size, number of lines
        println!("{}", self.info());
        // Prints and compares hashes of files
        if self.compare_hashes() {
            std::process::exit(0);
        }
        // Compares content of both files
        println!("{}", self.difr_include_empty_lines());
    }

    // Displays information about both files
    fn info(&self) -> String {
        let mut output = String::new();
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
        output = format!(
            "{}\n{}:\t{}\t{}{}\t{}{}",
            output,
            "File 1".bright_green(),
            &self.first_file_path.to_string_lossy().bright_cyan(),
            &size1.to_string().bright_cyan(),
            "bytes".bright_cyan(),
            &no_of_lines1.to_string().bright_cyan(),
            "line(s)".bright_cyan()
        );
        // """File 2:    file_name    size       no_of_lines"""
        output = format!(
            "{}\n{}:\t{}\t{}{}\t{}{}",
            output,
            "File 2".bright_green(),
            &self.second_file_path.to_string_lossy().bright_cyan(),
            &size2.to_string().bright_cyan(),
            "bytes".bright_cyan(),
            &no_of_lines2.to_string().bright_cyan(),
            "line(s)".bright_cyan()
        );

        output
    }

    // Prints and compares hashes of both files
    fn compare_hashes(&self) -> bool {
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
    fn difr_include_empty_lines(&mut self) -> String {
        let mut output = String::new();

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
                    output = format!(
                        "{}\n{} {} {}:\n{}\t{}",
                        output,
                        "Lines".bright_green(),
                        &current_line_no.to_string().bright_green(),
                        "-> End".bright_green(),
                        ">".bright_green(),
                        current_line1.bright_cyan()
                    );
                    while let Some(lines1_extra_line) = lines1.next() {
                        output = format!("{}\n\t{}", output, lines1_extra_line.bright_cyan());
                    }
                    output = format!(
                        "{}\n{}\t{}`{}`",
                        output,
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
                        output = format!(
                            "{}\n{} {}:\n{}\t{}\n{}\t{}\n",
                            output,
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
            output = format!(
                "{}\n{} {} {}:\n{}\t{}`{}`\n{}\t{}",
                output,
                "Lines".bright_green(),
                &current_line_no.to_string().bright_green(),
                "-> End".bright_green(),
                ">".bright_green(),
                "End of File for ".bright_cyan(),
                &self.first_file_path.to_string_lossy().bright_cyan(),
                ">".bright_green(),
                lines2_extra_line.bright_cyan()
            );
        }
        while let Some(lines2_extra_line) = lines2.next() {
            output = format!("{}\n\t{}", output, lines2_extra_line.bright_cyan());
        }

        output
    }

    fn is_text(pathbuf: &PathBuf) -> bool {
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

    fn hash(file_string: &str) -> String {
        let mut hasher = Sha3_256::new();

        // Write input message
        hasher.update(file_string.as_bytes());

        // Read hash digest
        let result = hasher.finalize();

        // Convert result from array to hex string
        hex::encode(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_whether_hash_is_correct() {
        assert_eq!(
            Difr::hash("What is your hash?"),
            "4de0afb7573ce206e62e02e261d48863e14bfccbc668dc0ea52be5616822016c"
        );
    }

    #[test]
    fn compare_hashes_of_equal_strings() {
        assert_eq!(
            Difr::hash("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            Difr::hash("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    #[should_panic]
    fn compare_hashes_of_separate_strings() {
        assert_eq!(
            Difr::hash("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            Difr::hash("ZYXWVUTSRQPONMLKJIHGFEDCBA")
        );
    }

    #[test]
    #[should_panic]
    fn check_if_image_file_is_a_text_file() {
        assert!(Difr::is_text(&PathBuf::from(
            "sample/Beaver_Wallpaper_Grey_4096x2304.png"
        )));
    }

    #[test]
    fn check_if_text_file_is_a_text_file() {
        assert!(Difr::is_text(&PathBuf::from("sample/foo.txt")));
    }

    #[test]
    fn difr_two_equal_files() {
        let mut app = Difr::init(
            PathBuf::from("sample/buz.txt"),
            PathBuf::from("sample/buz.txt"),
        );
        let expected_result = String::from("");

        assert_eq!(app.difr_include_empty_lines(), expected_result);
    }

    #[test]
    fn difr_two_different_files() {
        let mut app = Difr::init(
            PathBuf::from("sample/baz.txt"),
            PathBuf::from("sample/buz.txt"),
        );
        let expected_result = format!(
            "\n{} {}:\n{}\t{}\n{}\t{}\n",
            "Line".bright_green(),
            "1".bright_green(),
            ">".bright_green(),
            "Look at what you made me do.".bright_cyan(),
            ">".bright_green(),
            "What is your hash?".bright_cyan()
        );

        assert_eq!(app.difr_include_empty_lines(), expected_result);
    }
}
