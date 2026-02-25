// Docs: https://doc.rust-lang.org/std/fs/index.html

use std::fs;
use std::io::Result;

pub fn file_system() {
    if let Err(e) = copy_file("data.txt", "data1.txt") {
        println!("Cp-file: {e:?}");
    } else {
        println!("Cp-file: Sukses");
    }

    if let Ok(e) = cv_absolute_path("src/main.rs") {
        println!("Abs-path: {e:?}");
    }

    rename_file("data1.txt", "data2.txt").expect("Error rename");
}

// kalo berhasil ga return apapun dan klo error berikan pesan
// sebutannya unit type / mirip void
fn copy_file(from: &str, to: &str) -> Result<()> {
    fs::copy(from, to)?; // <- propagation: jika ada error langsung stop di baris ini
    Ok(())
}

fn cv_absolute_path(path: &str) -> Result<std::path::PathBuf> {
    let result = fs::canonicalize(path)?;
    Ok(result)
}

fn rename_file(from: &str, to: &str) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}
