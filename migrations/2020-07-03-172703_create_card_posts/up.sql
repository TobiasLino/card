-- Your SQL goes here
CREATE TYPE Addresstype as (
    cep VARCHAR(60),
    logradouro VARCHAR(60),
    complemento VARCHAR(60),
    bairro VARCHAR(60),
    localidade VARCHAR(60),
    uf VARCHAR(60),
    unidade VARCHAR(60),
    ibge VARCHAR(60),
    gia VARCHAR(60))
NOT FINAL

CREATE TABLE users (
    id INT(11) PRIMARY KEY AUTO_INCREMENT,
    'name' VARCHAR(60) NOT NULL,
    cpf VARCHAR(60) NOT NULL,
    phone VARCHAR(60) NOT NULL,
    email VARCHAR(60) NOT NULL,

    'address' AdressType,

    username VARCHAR(60) NOT NULL,
    passwd VARCHAR(60) NOT NULL,
    is_active BIT NOT NULL,
)