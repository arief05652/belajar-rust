// Docs: https://doc.rust-lang.org/std/process/index.html

use std::process::Command;

pub fn proses() {
    let cmd = Command::new("ls").output().unwrap();

    println!("Process: {:?}", String::from_utf8_lossy(&cmd.stdout));
}
