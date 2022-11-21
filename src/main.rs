use clap::Parser;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct Args {
    /// Path of the directory to scan, default current directory
    #[arg(short, long)]
    directory: Option<String>,
    /// File extension, default none
    #[arg(short, long)]
    file_extension: String,
}

fn visit(path: impl AsRef<Path>, cb: &mut dyn FnMut(PathBuf)) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let p = entry?.path();
        if p.metadata().unwrap().is_dir() {
            visit(p.as_path(), cb)?;
        } else {
            cb(p);
        }
    }
    Ok(())
}
fn main() {
    let args = Args::parse();
    let mut files = Vec::new();
    visit(
        Path::new(&args.directory.unwrap_or_else(|| String::from("."))),
        &mut |pb| {
            if pb.extension().unwrap_or_default().to_str().unwrap() == args.file_extension {
                let file = File::open(pb.as_path()).expect("Connat open file");
                let lines_count = BufReader::new(file).lines().count();
                files.push((pb, lines_count));
            }
        },
    )
    .unwrap();
    files.sort();

    for (file, lines) in files {
        println!("{} ({} lines)", file.as_path().display(), lines);
    }
}
