use mockall::*;
use mockall::predicate::*;

use std::{path::Path, convert::AsRef};

#[automock]
pub trait FileIO {
    fn exists<P: AsRef<Path> + 'static>(&self, path: P) -> bool;
}

pub fn find_nearest<F: FileIO>(config: &Config<F>) -> String {
    println!("file_io.exists('/') = {}", config.file_io.exists("/"));
    String::new()
}

pub struct Config<'a, F: FileIO> {
    file_io: &'a F,
    target_paths: Vec<&'a str>,
    starting_path: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut file_io = MockFileIO::new();
        file_io.expect_exists::<&str>().return_const(true);

        let config = Config {
            file_io: &file_io,
            target_paths: vec!["node_modules"],
            starting_path: "/home/user/code/project"
        };

        let result = find_nearest(&config);

        assert_eq!(result, "");
    }
}
