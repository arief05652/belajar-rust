/* borrowing: kasih pinjam untuk dibaca saat proses lain
 * terkadang membutuhkan data tanpa perlu membuatnya lagi
 * dan memakan banyak memory
 *
 * cara pakai:
 * - untuk yang readonly pakai `&` di awalan tipe datanya
 * - untuk yang mutable borrow pakai `&mut` dan merubah data aslinya
 */
pub fn borrow() {
    let mut a = String::from("borrow");
    pinjam(&a); // borrow read
    pinjam_mut(&mut a); // mutable borrow
    println!("read ori: {}", a);
}

fn pinjam(data: &String) {
    // readonly borrow
    println!("read borrow: {}", data)
}

fn pinjam_mut(data: &mut String) {
    data.push('!');
    println!("Borrow Mut: {}", data);
}
