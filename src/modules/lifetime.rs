pub fn lifetime() {
    let str1 = String::from("data_str1"); // masa hidup panjang
    let result;

    {
        let str2 = String::from("data_str2");
        // result ini diisi reference, dan ref ga akan bisa hidup lama kalau data aslinya
        // udah mati
        result = lifetime_example(str1.as_str(), str2.as_str());

        println!("Yang terpanjang adalah: {}", result);
    }
    // str2 disini mati karna beda scope
    // println!("{}", result)
}

/// return difungsi ini adalah reference lifetime "&'a str"
pub fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // ref
    } else {
        y // ref
    }
}
