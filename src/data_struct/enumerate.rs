#[derive(Debug)]
pub enum Access {
    Admin(String),
    Member(String),
}

#[derive(Debug)]
pub enum Gender {
    LakiLaki,
    Perempuan,
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub gender: Gender,
    pub status: Option<Access>,
}

impl Person {
    pub fn new(name: String, gender: Gender, status: Option<Access>) -> Option<Self> {
        match gender {
            Gender::LakiLaki => Some(Self {
                name,
                gender,
                status,
            }),
            Gender::Perempuan => None,
        }
    }

    // pinjam struct
    pub fn get_person(&self) -> &Person {
        self
    }

    pub fn get_gender(&self) -> &Gender {
        &self.gender
    }

    pub fn get_status(&self) -> &str {
        match &self.status {
            Some(Access::Admin(p)) | Some(Access::Member(p)) => p,
            _ => "None",
        }
    }
}
