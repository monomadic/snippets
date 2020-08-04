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
