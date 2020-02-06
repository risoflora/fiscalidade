//! Exemplo básico de como consultar _status_ do serviço.

extern crate anyhow;
extern crate fiscalidade;

use std::env;

use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Uf, WebServices};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Uso: <certificado.pfx> <senha> <uf> <ambiente>");
        return Ok(());
    }
    let webservices = WebServices::from_file("resources/webservices.ini")?;
    let pkcs12 = Pkcs12Certificate::from_file(&args[1], &args[2])?;
    let dfe = Dfe::new().set_webservices(webservices).set_pkcs12(pkcs12);
    let xml = dfe.status_servico(
        Uf::from_str(&args[3]).unwrap(),
        Ambiente::from_str(&args[4]).unwrap(),
    )?;
    println!("XML retornado: {}", String::from_utf8_lossy(&xml));
    Ok(())
}
