pub mod search_zip {
    use reqwest;
    use error_chain::error_chain;
    use std::collections::HashMap;
    use super::Address;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    #[tokio::main]
    async fn search_zip(zip: &str) -> Result<HashMap<String, String>> {
        let _url = format!("https://viacep.com.br/ws/{}/json", zip);
        // TODO: passar o parâmetro de entrada como valor no cep
        let res = reqwest::get("https://viacep.com.br/ws/12289456/json")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        Ok(res)
    }
    
    pub fn search(zip: &str) -> Address {
        //let mut zip = zip.matches(char::is_numeric);
        let addr = match search_zip(zip) {
            Ok(res) => res,
            Err(err) => {
                panic!("Erro na busca: {:?}", err)
            },
        };

        convert_map_into_address(addr)
    }

    fn convert_map_into_address(map: HashMap<String, String>) -> Address {
        Address {
            cep: map["cep"].to_string(),
            logradouro: map["logradouro"].to_string(),
            complemento: map["complemento"].to_string(),
            bairro: map["bairro"].to_string(),
            localidade: map["localidade"].to_string(),
            uf: map["uf"].to_string(),
            unidade: map["unidade"].to_string(),
            ibge: map["ibge"].to_string(),
            gia: map["gia"].to_string(),
        }
    }
}

pub struct Address {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub unidade: String,
    pub ibge: String,
    pub gia: String,
}

impl Address {
    pub const fn new() -> Address {
        Address {
            cep: String::new(),
            logradouro: String::new(),
            complemento: String::new(),
            bairro: String::new(),
            localidade: String::new(),
            uf: String::new(),
            unidade: String::new(),
            ibge: String::new(),
            gia: String::new(),
        }
    }
    pub fn data(&self) -> String {
        format!("{}\n{}", self.cep, self.logradouro)
    }
}

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
