pub mod user;


// test com reqwest = rq
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
}

// format!("{}\n{}", self.zip, self.street)