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


// `glob` crate
let options = glob::MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

let files = glob::glob_with("./*.md"), options)
    .map_err(|_| AstryxError::new("error globbing file"))?
    .map(|file| read_file(file.expect("file to unwrap")))
    .collect();

// write file
use std::fs::File;
use std::io::prelude::*;
fn write_file<S:ToString>(path: S, content: S) -> std::io::Result<()> {
    let mut file = File::create(path.to_string())?;
    file.write(content.to_string().as_bytes())?;
    Ok(())
}