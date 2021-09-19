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

- [x] Status do serviço
- [x] Consulta de cadastro
- [x] Consulta de XML
- [x] Configuração de webservices via arquivo INI
- [x] Leitura de certificado P12
- [x] Conexão segura usando biblioteca TLS _padrão_ do sistema
- [x] Configuração de _timeout_ da conexão e da comunicação com o webservice
- [x] Embutir arquivo de webservices na biblioteca
- [x] Atualização de webservices (`webservices.ini`)
- [x] Verificar se recursos (envelope, URLs etc.) estão atualizados
- [x] Chamadas async
- [ ] Validação e assinatura de XML
- [ ] Adicionar exemplo de consulta de cadastro
- [ ] Tentativas de comunicação com o webservice
- [ ] Mais serviços como envio de lote, consulta de recibo, inutilização,
      distribuição de DFe, etc.
- [ ] Compressão no envio de lote
- [ ] Testes
- [ ] Documentação
- [ ] Mais exemplos
- [ ] Proxy
- [ ] GitHub Actions
- [ ] DANFE

## Contribuições

Pull Requests e Issues são sempre bem-vindos! =)

## Licença

`fiscalidade` é distribuída sob qualquer uma das seguintes licenças:

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)
