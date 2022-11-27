use mockall::*;
use mockall::predicate::*;

use std::{path::Path, convert::AsRef};

#[automock]
pub trait FileIO {
    fn exists<P: AsRef<Path> + 'static>(&self, path: P) -> bool;
}

pub fn find_nearest(file_io: &impl FileIO) -> String {
    println!("file_io.exists('/') = {}", file_io.exists("/"));
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut file_io = MockFileIO::new();
        file_io.expect_exists::<&str>().return_const(true);

        let result = find_nearest(&file_io);

        assert_eq!(result, "");
    }
}
