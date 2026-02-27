use std::error::Error;
use std::fs;
use std::io;

// Struct recursive tidak bisa langsung menyimpan dirinya sendiri
// karena akan menyebabkan ukuran tipe menjadi tak terhingga.
// Rust harus mengetahui ukuran tipe saat compile time.
//
// Dengan menggunakan `Box`, data `Node` disimpan di heap,
// dan yang disimpan di dalam struct hanyalah pointer berukuran tetap.
// Karena pointer memiliki ukuran yang pasti,
// maka ukuran struct `Node` bisa dihitung oleh compiler.
struct Node {
    name: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn box_heap() -> Result<(), Box<dyn Error>> {
    tree_node();
    handle_error_with_box()?;
    handle_error_with_box1()?;
    Ok(())
}

fn tree_node() {
    let mut root = Node {
        name: "root_node".into(),
        left: Some(Box::new(Node {
            name: "left_node".into(),
            left: None,
            right: None,
        })),
        right: None,
    };

    // Mengakses isi Box (auto deref)
    if let Some(left) = root.left.as_ref() {
        println!("Sebelum diubah: {}", left.name);
    }

    // 🔥 Mengubah data di dalam Box
    if let Some(left) = root.left.as_mut() {
        left.name = "left_node_updated".into();
    }

    // as_ref: dikarnakan left itu bentuknya `Box` jadinya hanya
    //         meminjam tanpa memindahkan datanya, jika tidak maka ownership
    //         akan move.
    // as_mut: meminjam tapi dalam bentuk mutable jadi bisa merubah datanya
    //         tanpa perlu move ownership.

    if let Some(left) = root.left.as_ref() {
        println!("Setelah diubah: {}", left.name);
    }

    // Menaruh 1 juta angka 0 di heap
    let data = Box::new([0; 1_000_000]);

    // Jangan dereference ke stack karena akan memindahkan seluruh array.
    // Itu bisa menyebabkan stack overflow tergantung batas stack sistem.
    box_array(data);
}

// Parameter meminta Box yang berisi array
fn box_array(arr: Box<[i32; 1_000_000]>) {
    println!("Box-array first 5: {:?}", &arr[0..5]);
}

// Error handling dengan tipe konkret
fn handle_error_with_box() -> io::Result<()> {
    fs::write("test.txt", "test box error trait")?;
    Ok(())
}

// Trait `Error` tidak memiliki ukuran yang diketahui saat compile time.
// Dengan `dyn Error`, kita membuat trait object.
// Karena trait object tidak memiliki ukuran tetap,
// kita membungkusnya dengan `Box` agar memiliki ukuran pointer yang pasti.
// Data error disimpan di heap dan yang ada di stack hanyalah pointer-nya.
fn handle_error_with_box1() -> Result<(), Box<dyn Error>> {
    let isi = fs::read_to_string("angka.txt")?;
    let angka: i32 = isi.trim().parse()?;
    println!("Box error parse result: {}", angka);
    Ok(())
}
