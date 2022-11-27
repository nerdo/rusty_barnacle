use mockall::*;
use mockall::predicate::*;
use std::path::Path;

#[automock]
pub trait FileIO {
    fn exists(&self, path: &str) -> bool;
}

pub fn find_nearest<F: FileIO>(config: &Config<F>) -> String {
    let mut cur_path = Path::new(config.starting_path);

    loop {
        for p in config.target_paths.iter() {
            if let Some(test_path) = cur_path.join(p).to_str() {
                if config.file_io.exists(&test_path) {
                    return String::from(test_path);
                }
            }
        }

        cur_path = match cur_path.parent() {
            Some(parent) => parent,
            None => return String::new()
        };
    }
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
        file_io.expect_exists().return_const(true);

        let config = Config {
            file_io: &file_io,
            target_paths: vec!["node_modules"],
            starting_path: "/home/user/code/project"
        };

        let result = find_nearest(&config);

        assert_eq!(result, "");
    }
}
