//! Exemplo básico de como consultar _status_ do serviço.

extern crate anyhow;
extern crate fiscalidade;

use std::env;

use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Tipo, Uf, WebServices};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Uso: <certificado.pfx> <senha> <tipo> <uf> <ambiente>");
        return Ok(());
    }
    #[cfg(feature = "embed-webservices")]
    let webservices = WebServices::from_embedded()?;
    #[cfg(not(feature = "embed-webservices"))]
    let webservices = WebServices::from_file("resources/webservices.ini")?;
    let pkcs12 = Pkcs12Certificate::from_file(&args[1], &args[2])?;
    let dfe = Dfe::new(Tipo::from_str(&args[3]).unwrap_or(Tipo::Nfe))
        .set_webservices(webservices)
        .set_pkcs12(pkcs12);
    let xml = dfe.status_servico(
        Uf::from_str(&args[4]).unwrap(),
        Ambiente::from_str(&args[5]).unwrap(),
    )?;
    println!("XML retornado: {}", xml);
    Ok(())
}
