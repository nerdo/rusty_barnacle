use clap::Parser;
use std::{env, process, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to start in. Defaults to the current working directory
    #[arg(short, long, default_value_t = env::current_dir().unwrap().into_os_string().to_str().unwrap().to_string())]
    starting_path: String,

    /// The target path(s) to look for, e.g. node_modules/.bin
    #[arg(required = true)]
    target_paths: Vec<String>,

    /// Optional template to use when a match is found.
    /// Valid tokens (examples use node_modules/.bin as the matched target path):
    ///   {{.Path}}   The path containing the target path that was found
    ///   {{.Target}} The target path, i.e. {{.Path}}/node_modules/.bin
    #[arg(short, long, default_value_t = String::from("{{.Target}}"))]
    template: String,
}

struct RealFileIO {}

impl barnacle::FileIO for RealFileIO {
    fn exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}

fn main() {
    let cli = Cli::parse();

    let file_io = RealFileIO {};
    let config = barnacle::Config {
        file_io: &file_io,
        starting_path: cli.starting_path,
        target_paths: cli.target_paths,
        template: cli.template,
    };

    if let Some(result) = barnacle::find_nearest(&config) {
        println!("{}", result);
        process::exit(0);
    }

    process::exit(1);
}
