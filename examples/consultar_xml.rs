//! Exemplo básico de como consultar XML via chave da nota.

use std::env;

use fiscalidade::{Ambiente, Dfe, Modelo, Pkcs12Certificate, Tipo, Uf, WebServices};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        println!("Uso: <certificado.pfx> <senha> <tipo> <modelo> <uf> <ambiente> <chave>");
        return Ok(());
    }
    #[cfg(feature = "embed_webservices")]
    let webservices = WebServices::from_embedded()?;
    #[cfg(not(feature = "embed_webservices"))]
    let webservices = WebServices::from_file("resources/webservices.ini").await?;
    let pkcs12 = Pkcs12Certificate::from_file(&args[1], &args[2]).await?;
    let dfe = Dfe::new(Tipo::from_str(&args[3]).unwrap())
        .set_webservices(webservices)
        .set_pkcs12(pkcs12);
    let xml = dfe
        .consultar_xml(
            Modelo::from_str(&args[4]).unwrap(),
            Uf::from_str(&args[5]).unwrap(),
            Ambiente::from_str(&args[6]).unwrap(),
            &args[7],
        )
        .await?;
    println!("XML retornado: {}", xml);
    Ok(())
}
