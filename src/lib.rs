//! `fiscalidade` Biblioteca Rust para geração, validação, assinatura de XMLs de Documentos Fiscais e transmissão com webservices da SEFAZ.
//!
//! ## Exemplo
//!
//! O exemplo abaixo mostra como obter o _status_ do serviço de homologação para o Mato Grosso:
//!
//! ```rust
//! extern crate fiscalidade;
//!
//! use std::{error, result};
//!
//! use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Uf, WebServicesIni};
//!
//! fn main() -> result::Result<(), Box<dyn error::Error>> {
//!     let ini = WebServicesIni::from_file("resources/webservices.ini")?;
//!     let pkcs12 = Pkcs12Certificate::from_file("resources/certificado.pfx", "minha-senha-secreta")?;
//!     let dfe = Dfe::new().set_ini(ini).set_pkcs12(pkcs12);
//!     let xml = dfe.status_servico(Uf::Mt, Ambiente::Homologacao)?;
//!     println!("XML retornado: {}", String::from_utf8_lossy(xml.as_ref()));
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod dfe;
pub mod pkcs12;
pub mod soap;
pub mod tipos;
pub mod webservices;

pub use client::*;
pub use dfe::*;
pub use pkcs12::*;
pub use soap::*;
pub use tipos::*;
pub use webservices::*;
