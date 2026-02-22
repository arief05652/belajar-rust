// traits fungsi nya kaya abstract di oop tapi
// setiap impl yang memakai nya wajib membuat semua method
// yang ada di trait nya juga begitu pun semua tipe datanya

pub trait Suara {
    fn bersuara(&self) -> &str;
    fn set_suara(&mut self, suara: String);
}
