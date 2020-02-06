//! `fiscalidade` Biblioteca Rust para geração, validação, assinatura de XMLs de Documentos Fiscais e transmissão com webservices da SEFAZ.
//!
//! ## Exemplo
//!
//! O exemplo abaixo mostra como obter o _status_ do serviço de homologação para o Mato Grosso:
//!
//! ```rust
//! extern crate anyhow;
//! extern crate fiscalidade;
//!
//! use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Uf, WebServices};
//!
//! fn main() -> anyhow::Result<()> {
//!     let webservices = WebServices::from_file("resources/webservices.ini")?;
//!     let pkcs12 = Pkcs12Certificate::from_file("resources/certificado.pfx", "minha-senha-secreta")?;
//!     let dfe = Dfe::new().set_webservices(webservices).set_pkcs12(pkcs12);
//!     let xml = dfe.status_servico(Uf::Mt, Ambiente::Homologacao)?;
//!     println!("XML retornado: {}", String::from_utf8_lossy(&xml));
//!     Ok(())
//! }
//! ```

mod client;
mod dfe;
mod pkcs12;
mod soap;
mod tipos;
mod util;
mod webservices;

pub use client::*;
pub use dfe::*;
pub use pkcs12::*;
pub use soap::*;
pub use tipos::*;
pub use util::*;
pub use webservices::*;
