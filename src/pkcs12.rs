use std::{fs, io, path::Path, result};

use quick_error::quick_error;
use reqwest::Identity;

/// Objeto para manipulação de certificados PKCS #12.
#[derive(Debug)]
pub struct Pkcs12Certificate(Identity);

quick_error! {
    #[derive(Debug)]
    /// Tipo para tratar erros relacionados a I/O e leitura de certificado PKCS #12.
    pub enum Pkcs12CertificateError {
        /// Erros relacionados a I/O.
        Io(err: io::Error) {
            from()
            display("Erro de I/O ao carregar certificado: {}", err)
        }
        /// Erros relacionados a leitura de certificado PKCS #12.
        Pkcs12(err: reqwest::Error){
            from()
            display("Erro ao carregar certificado: {}", err)
        }
    }
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
    pub fn from_file<P: AsRef<Path>>(path: P, password: &str) -> Pkcs12CertificateResult {
        let bytes = fs::read(path)?;
        Self::from(&bytes, password)
    }

    pub(crate) fn into_inner(self) -> Identity {
        self.0
    }
}
