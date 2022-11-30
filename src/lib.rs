use mockall::*;
use mockall::predicate::*;
use gtmpl_derive::Gtmpl;
use std::path::Path;

#[automock]
pub trait FileIO {
    fn exists(&self, path: &str) -> bool;
}

pub struct Config<'a, F: FileIO> {
    file_io: &'a F,
    target_paths: Vec<&'a str>,
    starting_path: &'a str,
    template: &'a str,
}

#[derive(Gtmpl)]
#[allow(non_snake_case)]
struct NearestMatch {
    Path: String,
    Target: String,
}

pub fn find_nearest<F: FileIO>(config: &Config<F>) -> Option<String> {
    let init_path_buf = Path::new(config.starting_path).join("_");
    let mut cur_path = init_path_buf.as_path();

    while let Some(parent_path) = cur_path.parent() {
        cur_path = parent_path;

        for p in config.target_paths.iter() {
            if let Some(test_path) = cur_path.join(p).to_str() {
                if config.file_io.exists(&test_path) {
                    let nearest_match = NearestMatch {
                        Path: String::from(cur_path.to_str().unwrap()),
                        Target: String::from(test_path)
                    };

                    let output = gtmpl::template(config.template, nearest_match);
                    return Some(output.unwrap());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_when_it_does_not_find_a_target_path() {
        let mut file_io = MockFileIO::new();
        file_io.expect_exists().return_const(false);

        let config = Config {
            file_io: &file_io,
            target_paths: vec!["node_modules/.bin", ".custom_bin_path"],
            starting_path: "/home/user/code/project/src/app/components",
            template: ""
        };

        let result = find_nearest(&config);

        assert_eq!(result, None);
    }

    #[test]
    fn it_finds_the_nearest_path_where_target_paths_exist() {
        let mut file_io = MockFileIO::new();
        file_io.expect_exists().returning(|path| path == "/home/user/code/project/node_modules/.bin");

        let config = Config {
            file_io: &file_io,
            target_paths: vec!["node_modules/.bin", ".custom_bin_path"],
            starting_path: "/home/user/code/project/src/app/components",
            template: "Target: {{.Target}}, Path: {{.Path}}"
        };

        let result = find_nearest(&config);

        assert_eq!(result, Some(String::from("Target: /home/user/code/project/node_modules/.bin, Path: /home/user/code/project")));
    }
}
