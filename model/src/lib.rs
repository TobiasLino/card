pub mod user;
// test com reqwest = rq

// TODO: isolar os testes utilizando o mocktopus

#[cfg(test)]
mod tests {
    use super::user::search_zip::search;
    #[test]
    fn rq_addr_01() {
        let addr = search("12289456");
        assert_eq!(addr.cep, "12289-456");
        assert_eq!(addr.logradouro, "Avenida José Otávio de Macedo");
        assert_eq!(addr.complemento, "");
        assert_eq!(addr.bairro, "Jardim Panorama");
        assert_eq!(addr.localidade, "Caçapava");
        assert_eq!(addr.uf, "SP");
        assert_eq!(addr.unidade, "");
        assert_eq!(addr.ibge, "3508504");
        assert_eq!(addr.gia, "2343");
    }

    #[test]
    fn rq_addr_with_error_01() {
        let addr = search("12289-456");
        assert_ne!(addr.cep, "12289456");
    }
}

// format!("{}\n{}", self.zip, self.street)