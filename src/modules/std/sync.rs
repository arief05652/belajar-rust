use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;

pub fn sync_module() {
    mutex_sync();
    arc_sync();
    rwlock_sync();
    mpsc_sync();
}

// Mengunci data supaya hanya 1 thread yang bisa mengakses dalam satu waktu.
fn mutex_sync() {
    // Membuat Mutex yang menyimpan angka 10
    let data = Mutex::new(10);

    {
        // Mengambil lock (mengunci data)
        let mut hasil = data.lock().unwrap();

        // Dereference guard untuk mengubah nilai di dalam Mutex
        *hasil += 5;

        // Cetak nilai melalui guard
        println!("Di dalam lock: {}", *hasil);
    } // <-- Di sini lock otomatis dilepas (guard drop)

    // Lock lagi untuk membaca nilai akhir
    let final_value = data.lock().unwrap();
    println!("Setelah unlock: {}", *final_value);
}

// share data dengan berbeda thread
// biasa di pakai bareng mutex
fn arc_sync() {
    let data = Arc::new(10);
    let data2 = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("Arc-clone: {}, Arc: {}", *data2 + 2, data);
    });
    handle.join().unwrap();
}

// banyak read tapi cuman 1 tulis
fn rwlock_sync() {
    let data = Arc::new(RwLock::new(10));

    let mut handles = vec![];

    for _ in 0..3 {
        let d = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let r = d.read().unwrap();
            return format!("Read: {}", *r);
        });

        handles.push(handle);
    }

    for h in handles {
        let data = h.join().unwrap();
        println!("{data}")
    }
}

// konsep nya mirip arc+mutex tapi data hasil operasi di thread dikirim dulu ke channel
// baru pakai. kalo pakai arc+mutex data yang mau dipakai harus di lock supaya tidak race
fn mpsc_sync() {
    // Membuat channel (tx = pengirim, rx = penerima)
    let (tx, rx) = mpsc::channel();

    // Untuk menyimpan handle thread supaya bisa di-join nanti
    let mut handles = vec![];

    for data in 0..3 {
        // Clone transmitter supaya tiap thread punya tx sendiri
        let tx_clone = tx.clone();

        // Spawn thread baru
        let handle = thread::spawn(move || {
            // Kirim pesan ke channel
            // format! hanya untuk bikin string biar jelas outputnya
            tx_clone.send(format!("mpsc: {}", data)).unwrap();
        });

        // Simpan handle supaya bisa ditunggu selesai
        handles.push(handle);
    }

    // Drop tx utama supaya channel tahu tidak ada pengirim lagi
    // Channel akan tertutup setelah semua tx_clone juga selesai
    drop(tx);

    // Loop ini akan terus menerima pesan
    // sampai semua transmitter sudah di-drop
    for msg in rx {
        println!("Terima: {}", msg);
    }

    // Tunggu semua thread selesai eksekusi
    for handle in handles {
        handle.join().unwrap();
    }
}
