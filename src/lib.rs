use std::{fs::read_to_string, io::Read, path::PathBuf};

pub struct Difr {
    /// `PathBuf` of one of the two files to be compared
    file1: PathBuf,
    /// `PathBuf` of one of the two files to be compared
    file2: PathBuf,
    /// Whether to exclude empty lines or not
    exclude_empty_lines: bool,
    /// Line index to comparison begins. [Optional]
    from: Option<usize>,
    /// Line index to comparison ends. [Optional]
    to: Option<usize>,
}

impl Difr {
    pub fn init(
        file1: PathBuf,
        file2: PathBuf,
        exclude_empty_lines: bool,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Self {
        Difr {
            file1,
            file2,
            exclude_empty_lines,
            from,
            to,
        }
    }

    pub fn run(&self) {}

    pub fn difr_include_empty_lines(&self) {
        let content1 = read_to_string(&self.file1)
            .expect(format!("Unable to read file: {:?}", &self.file1).as_str());
        let content2 = read_to_string(&self.file2)
            .expect(format!("Unable to read file: {:?}", &self.file2).as_str());
    }

    pub fn difr_exclude_empty_lines(&self) {}
}

fn is_text(pathbuf: PathBuf) -> bool {
    let file = std::fs::File::open("./demo").expect("failed to open file");

    let mut buffer = Vec::with_capacity(32);
    file.take(32)
        .read_to_end(&mut buffer)
        .expect("failed to read from file");

    match std::str::from_utf8(&buffer) {
        Ok(_) => true,
        Err(_) => false,
    }
}
