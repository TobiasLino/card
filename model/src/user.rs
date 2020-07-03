pub mod address;
pub mod login;

use address::Address;
use login::Login;

// TODO: User precisa de um histórico

pub struct User {
    pub name: String,
    pub phone: String,
    pub address: Address,
    pub email: String,
    // dados de Login
    pub username: String,
    passwd: String,
    // dados de estado
    is_active: bool,
}

impl User {
    pub fn data(self) -> String {
        format!("{}\n\t{}", self.username.to_uppercase(), self.passwd.to_lowercase())
    }

    pub fn status(self) -> bool {
        self.is_active
    }
}

// TODO: Implementar essa interface
impl Login for User {
    pub fn get_user_by_username(username: String) -> Option<User> {

    }

    
    pub fn get_user_by_email(email: String) -> Option<User> {

    }

    pub fn check_passwd(self, passwd: String) -> bool {
        return self.passwd == passwd
    }

    pub fn connect(self) -> Result<bool> {

    }
}
