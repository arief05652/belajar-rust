// Docs: https://doc.rust-lang.org/std/collections/index.html

// Menyediakan struktur data yang siap pakai:
// - Sequences: Vec, VecDeque, LinkedList
// - Maps: HashMap, BTreeMap
// - Sets: HashSet, BTreeSet
// - Misc: BinaryHeap
//
// - BTree: semuanya di susun dan di sortir
// - BinaryHeap: mengurutkan element berdasarkan prioritas
// - VecDeque: bisa beroperasi dari 2 sisi

use std::collections::*;

pub fn collection() {
    let mut tree: BTreeMap<String, String> = BTreeMap::new();
    tree.insert("b".into(), "1".into());
    tree.insert("a".into(), "2".into());

    println!("Btree: {tree:?}");

    let mut bin_heap: BinaryHeap<u32> = BinaryHeap::new();
    bin_heap.push(10);
    bin_heap.push(20);

    println!("BinHeap: {:?}", bin_heap);

    println!("HeapPeek: {:?}", bin_heap.peek()); // angka terbesar

    bin_heap.pop(); // hapus angka terbesar

    println!("BinHeap: {:?}", bin_heap);
}
