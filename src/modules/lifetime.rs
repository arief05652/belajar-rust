pub fn lifetime() {
    let str1 = String::from("data_str1"); // --- Umur A (Panjang)
    let result; // variable kosong
    {
        let str2 = String::from("data_str2"); // --- Umur B (Pendek, cuma sampai "}")

        // Memanggil fungsi: Rust ngecek kontrak 'a.
        // Karena result bisa jadi str2, maka umur result "terikat" ke umur str2.
        result = lifetime_example(str1.as_str(), str2.as_str());

        println!("Yang terpanjang adalah: {}", result);
        // ^ AMAN: Karena str1 dan str2 dua-duanya masih hidup di sini.
    }
    // --- str2 MATI di sini.

    // Kalau kamu tulis println!("{}", result); di sini -> ERROR!
    // Karena Rust takut 'result' isinya data dari 'str2' yang sudah almarhum.
}

// ARTI KONTRAK 'a:
// "Gue bakal balikin referensi yang umurnya HANYA SEUMUR yang paling pendek
// di antara x atau y. Biar aman, gak ada yang megang data basi."
//
// ARTI KONTRAK &'static:
// kode tidak ada pernah di hapus selama program berjalan
pub fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
