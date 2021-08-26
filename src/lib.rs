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
//! use fiscalidade::{Ambiente, Dfe, Pkcs12Certificate, Tipo, Uf, WebServices};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> anyhow::Result<()> {
//!     let webservices = WebServices::from_embedded()?;
//!     let pkcs12 =
//!         Pkcs12Certificate::from_file("resources/certificado.pfx", "minha-senha-secreta").await?;
//!     let dfe = Dfe::new(Tipo::Nfe)
//!         .set_webservices(webservices)
//!         .set_pkcs12(pkcs12);
//!     let xml = dfe.status_servico(Uf::Mt, Ambiente::Homologacao).await?;
//!     println!("XML retornado: {}", xml);
//!     Ok(())
//! }
//! ```

extern crate ini;
extern crate regex;
extern crate reqwest;
extern crate thiserror;

mod client;
mod dfe;
mod pkcs12;
mod soap12;
mod tipos;
mod util;
mod webservices;

pub use crate::client::*;
pub use crate::dfe::*;
pub use crate::pkcs12::*;
pub use crate::soap12::*;
pub use crate::tipos::*;
pub use crate::util::*;
pub use crate::webservices::*;
