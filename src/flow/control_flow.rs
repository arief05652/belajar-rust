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
