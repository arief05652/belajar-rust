/* ownership yang berarti kepemilikan yang disini yaitu variable
 * ketika proses sudah selesai data tidak bisa di panggil keluar dari scope
 * akan tetapi bisa kalo di oper kepemilikan nya
 *
 * Note:
 * - tipe data yang bisa pindah kepemilikannya hanya yang disimpan di heap
 * - String, Vec<T>, HashMap, Box<T>, Struct/Enum
 */
pub fn owner() {
    let a = String::from("data");
    let c = a.clone(); // <- ini clone jadi bikin heap baru
    let b = a;

    println!("[Owner-move] {}", b);
    println!("[Owner-clone] {}", c);
}
