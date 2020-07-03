pub mod user;

use user::User;

pub trait Login {
    pub fn get_user_by_username(username: String) -> Option<User>;
    pub fn get_user_by_email(email: String) -> Option<User>;

    // compara a senha digitada com a do usuário atual
    pub fn check_passwd(self, passwd: String) -> bool;
    
    // O resultado desse result tornará verdadeiro a opção is_active
    pub fn connect(self) -> Result<bool>;
}