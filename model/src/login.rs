use crate::user::User;

pub trait Login {
    fn get_user_by_username(username: String) -> Option<User>;
    fn get_user_by_email(email: String) -> Option<User>;

    // compara a senha digitada com a do usuário atual
    fn check_passwd(self, passwd: String) -> bool;
    
    // O resultado desse result tornará verdadeiro a opção is_active
    fn connect(self) -> bool;
}