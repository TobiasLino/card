mod people {
    pub struct User {
        pub username: String,
        pub passwd: String,
    }
}

#[cfg(test)]
mod tests {
    use super::people::User;
    #[test]
    fn it_works() {
        let user = User {
            username: String::from("Tobias"),
            passwd: String::from( "2410"),
        };
        assert_eq!(user.username, "Tobias");
    }
}
