use std::{io, path::Path, result};

use tokio::fs;

use reqwest::Identity;
use thiserror::Error;

/// Objeto para manipulação de certificados PKCS #12.
#[derive(Debug)]
pub struct Pkcs12Certificate(Identity);

/// Tipo para tratar erros relacionados a I/O e leitura de certificado PKCS #12.
#[derive(Error, Debug)]
pub enum Pkcs12CertificateError {
    /// Erros relacionados a I/O.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Erros relacionados a leitura de certificado PKCS #12.
    #[error(transparent)]
    Pkcs12(#[from] reqwest::Error),
}

/// Tipo para tratar retorno de leitura do certificado PKCS #12.
pub type Pkcs12CertificateResult = result::Result<Pkcs12Certificate, Pkcs12CertificateError>;

impl Pkcs12Certificate {
    #[inline]
    fn from(bytes: &[u8], password: &str) -> Pkcs12CertificateResult {
        Ok(Self {
            0: Identity::from_pkcs12_der(&bytes, password)?,
        })
    }

    /// Cria novo objeto PKCS #12 a partir de bytes informando senha para descriptografar a chave.
    pub fn from_bytes(bytes: &[u8], password: &str) -> Pkcs12CertificateResult {
        Self::from(bytes, password)
    }

    /// Cria novo objeto PKCS #12 a partir de string estática informando senha para descriptografar a chave.
    pub fn from_str(string: &str, password: &str) -> Pkcs12CertificateResult {
        Self::from(string.as_bytes(), password)
    }

    /// Cria novo objeto PKCS #12 a partir de arquivo informando senha para descriptografar a chave.
    pub async fn from_file<P: AsRef<Path>>(path: P, password: &str) -> Pkcs12CertificateResult {
        let bytes = fs::read(path).await?;
        Self::from(&bytes, password)
    }

    pub(crate) fn into_inner(self) -> Identity {
        self.0
    }
}
