extern crate aio;

use aio::fs;
use aio::File;

fn main() {
    fs::mkdir(Path::new("./test_folder")).then(|_| {
        fs::touch(Path::new("./test_folder/file.txt"))
    }).then(|_| {
        "Hello!".pipe(File::write(Path::new("./test_folder/file.txt")))
    }).then(|_| {
        fs::rmrf(Path::new("./test_folder"))
    }).unwrap();
}

