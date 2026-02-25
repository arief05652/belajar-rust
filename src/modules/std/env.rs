// Docs: https://doc.rust-lang.org/std/env/index.html

// digunakan untuk ngakses environment dari sistem operasi.
// yang berarti:
// - ambil env yang ada di desktop bukan dari file .env
// - ambil argument cli pake `env::args`
// - ambil direktori yang ada di pc
// - ambil path execute

use std::env;

pub fn env() {
    let dir = env::current_dir().expect("error");
    println!("Dir: {dir:?}");

    for arg in env::args() {
        println!("Arg: {arg}");
    }
}
