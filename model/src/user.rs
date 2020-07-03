use serde::{Serialize, Deserialize};

use crate::address::Address;
//use crate::login::Login;

// TODO: User precisa de um histórico
#[derive(Serialize, Deserialize)]
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
        format!("{}\n\t{}\n\t{}\n\t{}\n\t{}\n", 
            self.name.to_uppercase(),
            self.phone,
            self.email,
            self.username,
            self.address.data())
    }

    pub fn status(self) -> bool {
        self.is_active
    }
}

// TODO: Implementar essa interface
// impl Login for User {
//     fn get_user_by_username(username: String) -> Option<User> {

//     }

    
//     fn get_user_by_email(email: String) -> Option<User> {

//     }

//     fn check_passwd(self, passwd: String) -> bool {
//         return self.passwd == passwd
//     }

//     fn connect(self) -> bool {

//     }
// }
