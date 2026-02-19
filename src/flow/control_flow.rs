pub fn flow() {
    let age: i8 = 18;
    let is_admin: bool = false;

    // if..else
    if is_admin && age >= 18 {
        println!("dia admin dan sudah dewasa");
    } else if is_admin && age < 18 {
        println!("dia admin dan tapi belum dewasa");
    } else {
        println!("dia bukan admin dan belum dewasa");
    }

    // short if..else
    let status = if is_admin { "Yes" } else { "No" };
    println!("Status: {}", status);

    // match
    match age {
        // kondisi
        15 => println!("masih remaja"),
        20 => println!("sudah dewasa"),
        // default match
        _ => println!("default match"),
    }

    // multiple match + return value
    let age_range = match age {
        10..=17 => "kecil",
        18..=24 => "dewasa",
        _ => "tua",
    };
    println!("Multiple: {}", age_range);
}

/* ERROR HANDLING
 * - Result<T, E>: dipakai untuk operasi yang bisa jalan dan tidak
 *                 dipakai bersama `Ok()` dan `Err()`
 * - Option<T>: dipakai untuk operasi yang bisa ada atau none
 *              pakai `Some()` kalau ada nilainya dan `None` kalau tidak ada
 * - .unwrap(): kalo ada datanya ambil tapi kalo ga ada crash
 * - .expect(): sama kaya unwrap tapi bisa dikasih pesan costom
 */
pub fn result_handling(angka1: i16, angka2: i16) -> Result<i16, String> {
    if angka1 == 0 && angka2 == 0 {
        Err(String::from("kedua nya tidak boleh nol"))
    } else {
        Ok(angka1 + angka2)
    }
}

pub fn option_handling(user: &str) -> Option<String> {
    if user.is_empty() {
        None
    } else {
        Some("option running".to_string())
    }
}
