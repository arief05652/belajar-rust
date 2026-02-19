pub fn looping() {
    let mut counter = 1;

    /* Loops
     * - ini selalu jalan sampai waktunya berhenti
     * - entah pakai `break` atau `Ctrl + C`
     */
    loop {
        println!("Counter({})", counter);
        if counter == 3 {
            break; // stop loop
        }
        counter += 1;
    }

    /* Loops dengan return
     * - waktu break harus memakai variable nya
     */
    counter = 1;
    let looping_data = loop {
        println!("CounterReturn({})", counter);

        if counter == 3 {
            break counter;
        }

        counter += 1;
    };
    println!("total loop: {}", looping_data);

    /* While Loops
     * - range nya sudah ditentukan
     */
    counter = 1;
    while counter <= 3 {
        // skip angka tertentu
        if counter == 2 {
            // cegah infinite dan angkanya dan
            // angkanya langsung balik ke atas lagi
            counter += 1;
            continue; // <- skip angka sekarang
        }

        println!("WhileLoop: {}", counter);

        counter += 1;
    }

    /* For Loops
     * - lebih baik pakai ini karna untuk cegah infinite loop
     *   jika lupa increment
     * - lebih safety
     */
    for num in 1..=3 {
        if num == 2 {
            continue;
        }
        println!("ForLoops: {}", num);
    }
}

pub fn dummy_function() {}
