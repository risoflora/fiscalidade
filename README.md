# `fiscalidade`

[WIP] Biblioteca Rust para validação, assinatura e transmissão de XMLs para
webservices SEFAZ.

## Exemplo

O exemplo abaixo mostra como obter o _status_ do serviço de homologação para o
Mato Grosso:

```rust
extern crate anyhow;
extern crate fiscalidade;

use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Tipo, Uf, WebServices};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let webservices = WebServices::from_embedded()?;
    let pkcs12 =
        Pkcs12Certificate::from_file("resources/certificado.pfx", "minha-senha-secreta").await?;
    let dfe = Dfe::new(Tipo::Nfe)
        .set_webservices(webservices)
        .set_pkcs12(pkcs12);
    let xml = dfe.status_servico(Uf::Mt, Ambiente::Homologacao).await?;
    println!("XML retornado: {}", xml);
    Ok(())
}
```

## Uso

Adicione isto em seu `Cargo.toml`:

```ini
[dependencies]
tokio = { version = "1", features = ["full"] }
fiscalidade = "0.5"
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
- [x] atualização de webservices (`webservices.ini`)
- [x] verificar se recursos (envelope, URLs etc.) estão atualizados
- [x] chamadas async
- [ ] validação e assinatura de XML
- [ ] adicionar exemplo de consulta de cadastro
- [ ] tentativas de comunicação com o webservice
- [ ] mais serviços como envio de lote, consulta de recibo, inutilização,
      distribuição de DFe, etc.
- [ ] compressão no envio de lote
- [ ] testes
- [ ] documentação
- [ ] mais exemplos
- [ ] DANFE
- [ ] GitHub Actions

## Contribuições

Pull Requests e Issues são sempre bem-vindos! =)

## Licença

`fiscalidade` é distribuída sob qualquer uma das seguintes licenças:

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)
