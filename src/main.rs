use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct RunnablesCli {
    path: String,
}

fn main() {
    let mut paths = Vec::<String>::new();
    find_cargo_tomls(".".into(), &mut paths);
    println!("{paths:#?}");
}



fn find_cargo_tomls(pathbuf: PathBuf, paths: &mut Vec<String>) {
    let stuffs = fs::read_dir(pathbuf).unwrap();

    for stuf in stuffs {
        match stuf {
            Ok(thing) => {
                let path = thing.path().display().to_string();
                if path.contains("Cargo.toml") {
                    paths.push(path);
                }
                if thing.metadata().unwrap().is_dir() {
                    find_cargo_tomls(thing.path(), paths);     
                }
            }
            Err(e) => {
                println!("{e:#?}")
            }
        }
    }
}
