// Docs: https://doc.rust-lang.org/std/path/struct.Path.html

use std::path::{Path, PathBuf};

pub fn path_manipulation() {
    // path reference, cuman baca aja
    let regist = regist_path_ref("data.txt");
    println!("Path-ref: {regist:?}");

    // path ownership
    let mut regist_owner = regist_path_owner(String::from("data/"));
    println!("Path-owner: {regist_owner:?}");
    regist_owner.push("ucup");
    regist_owner.add_extension("txt");
    println!("Path-owner: {regist_owner:?}");
}

// `Path::new()` hanya reference
fn regist_path_ref(path: &str) -> &Path {
    Path::new(path)
}

// `PathBuf::from()` bisa nambah, hapus dll
fn regist_path_owner(path: String) -> PathBuf {
    PathBuf::from(path)
}
