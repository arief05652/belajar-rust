// Docs: https://doc.rust-lang.org/std/thread/index.html

use std::thread;

pub fn threading() {
    // menjalankan proses di thread berbeda
    let th = thread::spawn(|| {
        println!("print on another thread");
    });
    th.join() // join menghasilkan result jadi harus di handle
        .unwrap(); // handle ketika ada error di thread langsung matikan proses

    // #######################

    // jika ingin menjalankan thread tetapi ingin memakai data dari thread utama
    // jadi harus memindahkan kepemilikan datanya ke thread yang menjalankan nya
    //
    // jika tidak di `move` rust akan mencoba meminjam saja dan itu tidak bisa
    // karna untuk mencegah race condition
    let send = 50;
    let th1 = thread::spawn(move || send + 5);
    // untuk mengambil result dari thread
    let result = th1.join().expect("error proses");
    println!("data dari thread lain: {result}");

    // #######################

    // mengumpulkan hasil thread ke dalam vector
    let mut vec_thread = vec![];

    for th in 1..4 {
        let thrd = thread::spawn(move || {
            println!("thread loop: {th}");
        });

        vec_thread.push(thrd);
    }

    // menjalankan thread dari looping vector
    for vec in vec_thread {
        vec.join().unwrap();
    }
}
