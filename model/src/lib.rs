pub mod user;
pub mod address;
pub mod login;
// test com reqwest = rq

// TODO: isolar os testes utilizando o mocktopus

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        assert_eq!(2, 1+1);
    }
}

// format!("{}\n{}", self.zip, self.street)