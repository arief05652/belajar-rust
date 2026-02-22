use std::collections::HashMap;

/* Note:
 * - untuk `HashSet` konsep nya sama kaya array & vector
 *   cuman tidak boleh duplikat dan bisa dimasukan data apa aja
 *   misal struct.
 * - contoh nested hashmap: HashMap<String, HashMap<String, f32>>
 */
pub fn hashmap() {
    // bisa pakai with_capacity(), untuk kasih limit
    // jika data yang dipakai tidak sampai maks capacity
    // pakai `.shrink_to_fit()`, untuk menyesuaikan capacity dengan data
    let mut data: HashMap<String, i32> = HashMap::new();
    // masukan data
    data.insert(String::from("blue"), 10);
    data.insert(String::from("blue"), 30); // key ga boleh sama dan akan di ignore

    println!("hashmap: {data:?}");
    println!("hashmap-len: {}", data.len()); // cek panjang
    match data.get("blue") {
        // get
        Some(data) => println!("data-get: {data}"),
        None => println!("data-get: tidak ada data"),
    }
    for (key, value) in data.iter() {
        // iterate map
        println!("map-iter: {key} - {value}");
    }
    // data.remove("blue"); // hapus key
    if data.contains_key("blue") {
        // cek key
        println!("key ada");
    }

    /* Entry
     * - cek data sekali jalan lalu manipulasinya ditempat
     * - lebih cepat proses nya dari pada cek contain
     * - dipakai dengan chaining
     *
     * Method:
     * - or_insert: kalo ga ada masukan dengan value, kalo ada
     *              di abaikan dan kembalikan yang aslinya
     * - or_insert_with: hemat cpu, jika data default nya sudah ada
     *                   skip menjalankan fungsi yang berat untuk mengisi
     *                   default valuenya
     * - and_modify: kalau datanya ada lakukan sesuatu
     * - or_default: kalau datanya kosong isi dengan value minimal dari tipe datanya
     */

    data.entry("purple".to_string()).or_default();
    data.entry("red".to_string()).or_insert(15);
    data.entry("brown".to_string()).or_insert_with(|| {
        let data = 10;
        data
    });
    data.entry("black".to_string())
        .and_modify(|e| *e += 50)
        .or_insert(25); // kalau key ga ada isi dengan value ini
    let pink = data.entry("pink".to_string()).or_insert(34);
    *pink += 10; // bisa menambahkan data kaya gini pake pointer

    println!("Hash: {data:?}");
}
