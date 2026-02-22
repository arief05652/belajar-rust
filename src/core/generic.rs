use crate::core::traits::Suara;

// generic bisa membuat code menjadi flexible dengan berbagai tipe data
// akan tetapi data yang masuk dan keluar harus 1 tipe

// GENERIC IN STRUCT
#[derive(Debug)]
struct Player<T> {
    username: T,
}

// ENUM GENERIC
enum Result<T> {
    RESULT(T),
    NONE,
}

// IMPLEMENT GENERIC
// impl generic
impl<T> Player<T> {
    pub fn get_username(&self) -> &T {
        &self.username
    }
}

// GENERIC IN FUNC
fn func_generic<T>(data: &T) -> &T {
    data
}

// GENERIC BOUND IN FUNC BUT WITH WHERE CLAUSE
// pake generic tapi dibatesin dengan trait
fn func_generic_where<A, B>(data1: A, data2: B)
where
    A: PartialOrd + PartialEq,
{
}

#[derive(Debug)]
struct Kucing {
    nama: String,
    suara: Option<String>,
}

impl Suara for Kucing {
    fn bersuara(&self) -> &str {
        &self.nama
    }

    fn set_suara(&mut self, suara: String) {
        if !suara.is_empty() {
            self.suara = Some(suara)
        }
    }
}

pub fn generic_type() {
    // bisa di inisiasi pake tipe apa aja
    let player = Player { username: "arief" };
    // let player1: Player<i8> = Player { username: 127 };
    player.get_username();

    println!("Generic-Struct: {:?}", player);

    let mut kucing = Kucing {
        nama: String::from("Molly"),
        suara: None,
    };
    kucing.set_suara(String::from("Miaw"));
    println!("kucing: {:?}", kucing.suara.unwrap());
}
