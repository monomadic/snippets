/*

# Paths vs PathBuf

PathBuf is owned, Path is unowned.
&str would be equivalent to &Path, PathBuf would be String. Generally you want to pass around PathBuf

*/

// `walkdir` crate
use walkdir::WalkDir;
for entry in WalkDir::new("foo") {
    println!("{}", entry?.path().display());
}

//
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn readfile(pathbuf: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut f = File::open(pathbuf.clone())?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
