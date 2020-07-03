pub mod address;

use address::Address;

pub struct User {
    pub name: String,
    pub phone: String,
    pub address: Address,
    pub email: String,
    pub username: String,
    pub passwd: String
}

impl User {
    pub fn data(self) -> String {
        format!("{}\n\t{}", self.username.to_uppercase(), self.passwd.to_lowercase())
    }
}
