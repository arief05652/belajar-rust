mod data_struct;
mod core;
mod function;
mod operator;
mod strings;
mod flow; // <- module / nama file tempat register

/* - panggil nama file nya dengan spesifik `use flow::control_flow::{pub};`
 * - panggil semua nya `use flow::{mod} / *::{pub} / *;`
 * - panggil sesuai grup dan juga di kasih alias `use flow::{loops as l, control_flow as cf};`
 * - jika ingin 1 tipe nya fix dan yang lainnya bisa di panggil bisa pakai grup+self `use flow::loops::{self, looping};`
 * - jika ingin merujuk ke root path bisa pakai crate `use crate::` posisi nya langsung main file
 */
use flow::control_flow::{flow, result_handling, option_handling};
use flow::loops::{self, looping};
use core::{ownership, borrowing};
use data_struct::structs::{User, Person};
use data_struct::enumerate::{Person as p, Gender, Access};
use data_struct::vector::vector_heap;

use std::{f32, i8, u8};

fn main() {
    // OUTPUT
    println!("cetak dengan baris baru"); // cetak dengan baris baru
    print!("cetak biasa\n"); // cetak tanpa membuat baris baru. "\n" <- untuk bikin newline

    // VARIABLE
    let a: i8 = 10; // <- nilai nya selalu tetap
    let mut b: i8 = 0; // <- bisa di ubah
    /*
     * "{}" tanda ini di print berguna seperti place holder
     * yang dapat memasukan data
     */
    println!("a: {} - b: {}", a, b);
    b = 15;
    println!("a: {} - b change: {}", a, b);

    // TIPE DATA
    /*
     * integer: i8 -> i128 = dimulai dari -n -> n, bisa negatif dan positif
     * unsingned: u8 -> u128 = dimulai dari 0 -> n, bisa positif saja
     * float: f16 -> f128 = dipakai jika butuh desimal
     * string: &str = string dan harus pakai kutip 2 ""
     * char: char = pakai kutip 1 '', digunakan saat bekerja dengan
     *              unicode dan karakter tunggal
     * boolean: bool = true/false
     *
     * compound:
     * - tuple: digunakan untuk mengelompokan tipe data yang beda2 di satu tempat
     * - array: sama kaya tuple tapi tipe datanya sama dan panjang nya fix
     */
    let c: i8 = i8::MAX;
    let d: u8 = u8::MIN;
    let e: f32 = f32::MAX;
    let f: &str = "string";
    let g: char = '😁';
    let h: bool = true;
    let tuple: (u8, f32) = (15, 10.5);
    let array: [i32; 3] = [200, 194, 4124];

    println!("integer: {}", c);
    println!("unsigned: {}", d);
    println!("float: {}", e);
    println!("string: {}", f);
    println!("char: {}", g);
    println!("bool: {}", h);
    println!("tuple: {}", tuple.1); // ambil datanya pake .{index}
    println!("array: {}", array[1]); // ambil datanya pake [index]

    // destructuring, untuk bongkar struktur data
    match tuple {
        (x, y) => println!("tuple({}, {})", x, y),
    }

    /* CONSTANT
     * tidak bisa di re-assign dan harus pake tipe
     */
    const TOKEN: &str = "1jsdkajsmd012esjdm";
    println!("token: {}", TOKEN);

    // OPERATOR
    operator::operator();

    // CONTROL FLOW
    flow();

    // LOOPING
    looping(); // <- pakai langsung
    loops::dummy_function(); // <- dari self group

    // FUNCTION
    function::functions("ucup");

    // STRING
    strings::string();

    // ERROR HANDLING
    let result = result_handling(12, 0);
    match result {
        Ok(hasil) => println!("Result Handling: {}", hasil),
        Err(pesan) => println!("Result Handling: {}", pesan),
    }

    // kalau butuh handle tapi cuman 1 kondisi aja pake `if let`
    // if let Ok(hasil) = result {
    //     println!("Result Handling: {}", hasil);
    // }

    let option = option_handling("saepul");
    // let option = option_handling("saepul").unwrap(); <- via unwrap kalo kosong langsung crash tanpa info
    // let option = option_handling("saepul").expect("error kosong"); <- via expect sama kaya unwrap tapi bisa custom info
    match option { // <- handling option via match
        Some(pesan) => println!("Option: {}", pesan),
        None => println!("Option None"),
    }

    /* TYPE CASTING
     * - angka -> angka: pakai as
     * - str -> angka: pakai .parse(), dan mengembalikan `Result`
     * - &str -> String: pakai .into()
     * - String -> &str: pakai .as_str()
     */
     let x = 10;
     let y: f32 = x as f32; // <- pake as
     println!("angka -> angka: {}", y);

     let x: &str = "43";
     let y: u8 = x.parse().unwrap();
     println!("&str -> angka: {}", y);

     let x: &str = "dummy";
     let y: String = x.into();
     println!("&str -> String: {}", y);

     let x: String = String::from("dummys");
     let y: &str = x.as_str();
     println!("String -> &str: {}", y);

     // OWNERSHIP
     ownership::owner();

     // BORROWING
     borrowing::borrow();

     // STRUCT & IMPL
     let orang = Person{jenis: String::from("male")};
     // pakai mutable kalau ada operasi yang butuh perubahan
     let mut username = User::new(String::from("ucup"), 19, orang);

     // ganti value struct
     username.ganti_jenis(String::from("female"));

     username.person();
     println!("Name: {}", username.get_name());

     // ENUMERATE
     let status = Some(Access::Admin(String::from("admin")));
     let person_enum = p::new(String::from("Haikal"), Gender::LakiLaki, status);

     if let Some(data) = person_enum {
         let status = data.get_status();
         let gender = data.get_gender();
         let person = data.get_person();
         println!("Person-Person: {:?}", person);
         println!("Person-Gender: {:?}", gender);
         println!("Person-Status: {}", status);
     }
     // VECTOR
     vector_heap();
}
