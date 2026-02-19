mod function;
mod operator;
mod flow; // <- module / nama file tempat register

/* - panggil nama file nya dengan spesifik `use flow::control_flow::{pub};`
 * - panggil semua nya `use flow::{mod} / *::{pub} / *;`
 * - panggil sesuai grup dan juga di kasih alias `use flow::{loops as l, control_flow as cf};`
 * - jika ingin 1 tipe nya fix dan yang lainnya bisa di panggil bisa pakai grup+self `use flow::loops::{self, looping};`
 * - jika ingin merujuk ke root path bisa pakai crate `use crate::` posisi nya langsung main file
 */
use flow::control_flow::flow;
use flow::loops::{self, looping};

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
     */
    let c: i8 = i8::MAX;
    let d: u8 = u8::MIN;
    let e: f32 = f32::MAX;
    let f: &str = "string";
    let g: char = '😁';
    let h: bool = true;
    println!("integer: {}", c);
    println!("unsigned: {}", d);
    println!("float: {}", e);
    println!("string: {}", f);
    println!("char: {}", g);
    println!("bool: {}", h);

    // CONSTANT
    /*
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

    // function
    function::functions("ucup");
}
