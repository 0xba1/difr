use difr::Difr;
use std::path::PathBuf;

fn main() {
    let mut app = Difr::init(
        PathBuf::from("sample/bar.txt"),
        PathBuf::from("sample/foo.txt"),
        false,
        None,
        None,
    );
    app.run();
}
