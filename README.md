# `fiscalidade`

[WIP] Biblioteca Rust para geração, validação, assinatura de XMLs de Documentos Fiscais e transmissão com webservices da SEFAZ.

## Exemplo

O exemplo abaixo mostra como obter o _status_ do serviço de homologação para o Mato Grosso:

```rust
extern crate anyhow;
extern crate fiscalidade;

use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Tipo, Uf, WebServices};

fn main() -> anyhow::Result<()> {
    let webservices = WebServices::from_file("resources/webservices.ini")?;
    let pkcs12 = Pkcs12Certificate::from_file("resources/certificado.pfx", "minha-senha-secreta")?;
    let dfe = Dfe::new(Tipo::Nfe)
        .set_webservices(webservices)
        .set_pkcs12(pkcs12);
    let xml = dfe.status_servico(Uf::Mt, Ambiente::Homologacao)?;
    println!("XML retornado: {}", String::from_utf8_lossy(&xml));
    Ok(())
}
```

## Uso

Adicione isto em seu `Cargo.toml`:

```ini
[dependencies]
fiscalidade = "0.3.0"
```

e isto em seu _crate root_:

```rust
extern crate fiscalidade;
```

## _Wishlist_

- [x] status do serviço
- [x] consulta de cadastro
- [x] consulta de XML
- [x] configuração de webservices via arquivo INI
- [x] leitura de certificado P12
- [x] conexão segura usando biblioteca TLS _padrão_ do sistema
- [x] configuração de _timeout_ da conexão e da comunicação com o webservice
- [x] embutir arquivo de webservices na biblioteca
- [ ] atualização de webservices (`webservices.ini`)
- [ ] validação e assinatura de XML
- [ ] tentativas de comunicação com o webservice
- [ ] chamadas async
- [ ] mais serviços como envio de lote, consulta de recibo, inutilização, distribuição de DFe, etc.
- [ ] compressão no envio de lote
- [ ] testes
- [ ] documentação
- [ ] mais exemplos
- [ ] geração de XML
- [ ] DANFE

## Contribuições

Pull Requests e Issues são sempre bem-vindos! =)

## Licença

`fiscalidade` é distribuída sob qualquer uma das seguintes licenças:

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
