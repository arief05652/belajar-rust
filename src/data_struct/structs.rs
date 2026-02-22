/* struct dipakai untuk membuat tipe data baru yang key dan value nya bisa ditentukan
 * fungsinya mirip class di oop, di struct semuanya bersifat private harus pake keyword
 * `pub` jika mau pakai dari luar scope.
 *
 * jika mau mengubah value disuatu field bisa pakai `mut`, let mut user = User {}
 * kalo mau mengambil value nya aja pakai let user = User {}
 */
#[derive(Debug)]
pub struct Person {
    pub jenis: String,
}

#[derive(Debug)]
// digunakan jika ingin cetak struct langsung pakai
// `{:?}` sebagai placeholder
pub struct User {
    name: String,
    age: i32,
    person: Person, // nested struct
}

/* implement dipakai jika ingin melakukan operasi kompleks
 * misal bikin method new untuk memasukan ke dalam struct
 * dan menjadikan semua field beroperasi melalui method dan struct
 * tetap private
 */
impl User {
    pub fn new(username: String, age: i32, person: Person) -> Self {
        // `Self` merujuk nama struct,
        // kalau parameter sama dengan field bisa langsung masuk
        Self {
            name: username, // contoh dengan named argument
            age,
            person,
        }
    }

    // `&self` akan memberikan alamat memory untuk dipinjam dan dibaca
    pub fn person(&self) {
        println!(
            "name: {}, age: {}, jenis: {}",
            self.name, self.age, self.person.jenis
        );
    }

    // `self` akan memindahkan kepemilikan dan yang di struct akan hancur
    // atau bisajuga di clone heapnya tapi ada cost pastinya
    pub fn get_name(self) -> String {
        self.name
    }

    // kalau mau mengganti value struct bisa dengan meminjam dengan mutable
    // atau dengan menghasilkan object baru. Contoh:
    //
    // # replace dengan struct baru
    // pub fn ganti_jenis(&mut self, person_baru: Person) {
    //   self.person = person_baru
    // }
    //
    pub fn ganti_jenis(&mut self, jenis: String) {
        self.person.jenis = jenis
    }
}
