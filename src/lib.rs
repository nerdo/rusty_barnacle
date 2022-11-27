use mockall::*;
use mockall::predicate::*;
use std::path::Path;

#[automock]
pub trait FileIO {
    fn exists(&self, path: &str) -> bool;
}

pub fn find_nearest<F: FileIO>(config: &Config<F>) -> String {
    let init_path_buf = Path::new(config.starting_path).join("_");
    let mut cur_path = init_path_buf.as_path();

    while let Some(parent_path) = cur_path.parent() {
        cur_path = parent_path;

        for p in config.target_paths.iter() {
            if let Some(test_path) = cur_path.join(p).to_str() {
                if config.file_io.exists(&test_path) {
                    return String::from(test_path);
                }
            }
        }
    }

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
    fn it_finds_the_nearest_path_where_target_paths_exist() {
        let mut file_io = MockFileIO::new();
        let mut exists_values = vec![true, false];
        file_io.expect_exists().returning(move |_| exists_values.pop().unwrap_or(false));

        let config = Config {
            file_io: &file_io,
            target_paths: vec!["node_modules"],
            starting_path: "/home/user/code/project/subdir"
        };

        let result = find_nearest(&config);

        assert_eq!(result, "/home/user/code/project/node_modules");
    }
}
