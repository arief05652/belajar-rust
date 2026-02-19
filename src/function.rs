// Return type pakai `String` karena kita membuat data baru di dalam fungsi (dinamis).
// `&str` hanya referensi (pinjaman), dia tidak bisa berdiri sendiri jika datanya baru dibuat.
// `String` adalah pemilik data (owner) yang dialokasikan di Heap.
pub fn functions(a: &str) -> String {
    // Pakai format! karena kita menggabungkan string (f-string nya Rust)
    // Di sini 'return' boleh dipakai, tapi gaya Rust lebih suka tanpa 'return' dan tanpa ';'
    format!("dari file function.rs: {}", a)
}
