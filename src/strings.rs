pub fn string() {
    // deklasarinya bisa pakai `to_string()` atau `String::from()`
    let mut username = String::from("arief");
    username.push_str(" minardi "); // <- masukan kata
    username.push('😎');

    println!("String: {} | Len: {}", username, username.len());

    // SHADOWING
    // teknik ini berguna untuk meminimalkan nama variable yang banyak
    // karna x yang lama akan di tutup aksesnya lalu value nya dipindahkan ke x yang baru
    // dan bisa mengubah tipe data langsung kalau pakai `mut` kan hanya value nya ada yang berubah
    let x = 5;
    let x = x + 20; // <- shadowing
    println!("Shadowing variable: {}", x);
}
