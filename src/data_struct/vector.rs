pub fn vector_heap() {
    let mut data = vec![1, 2, 3]; // vector dengan macro
    let mut data1: Vec<i32> = Vec::new(); // vector dengan struct, lebih flexible
    data1.push(4);

    // masukan data
    data.push(4); // masukan di baris terakhir
    data.insert(0, 0); // masukan sesuai index
    data.extend([1, 2, 3]); // bisa memasukan array, vector, slice
    // untuk menambahkan vector, dan vector yang 1 nya akan terkuras
    // `&mut` harus pakai ini
    data.append(&mut data1);

    data.remove(4); // hapus sesuai index
    data[0] = -1; // ganti value sesuai index
    let a = &data[1]; // ambil value

    println!("vector: {data:?}");
    println!("vector-access: {a}");

    for v in &mut data {
        // tanda `*` adalah deferensi yang beguna untuk menunjuk pointer memori
        // dan merubahnya dari `&T` menjadi `T`
        *v += 10;
    }

    // pop data indeks terakhir
    data.pop();

    println!("Vec: {data:?}");

    // ambil data menggunakan .get agar jika index tidak ada program tidak langsung crash
    let take_data = data.get(1).expect("data tidak ada");
    println!("Get-Vec: {take_data}");

    // slicing
    let slice = &data[0..2];
    println!("Slice-Vec: {slice:?}");

    // limitasi panjang vector
    // cocok dipakai jika sudah tau berapa maksnya
    // dan rust sudah memesan block memory terlebih dahulu
    let mut data3: Vec<i32> = Vec::with_capacity(20);
    println!("Vector-capacity: {}", data3.capacity());

    // remove tapi swap
    // angka yang di hapus akan di ganti dengan indeks terakhir
    data3.extend([1, 2, 3]);
    data3.swap_remove(0);
    println!("swap-remove: {data3:?}");

    // cek data tanpa mengambilnya
    if data3.contains(&2) {
        println!("angka 3 ada di vector");
    }
}
